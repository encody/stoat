use std::{borrow::Cow, ops::Deref};

use pulldown_cmark::{Event, Tag};
use serde::Deserialize;

pub trait TextContent {
    fn text_content(&self) -> Cow<'_, str>;
}

impl<T: TextContent> TextContent for [T] {
    fn text_content(&self) -> Cow<'_, str> {
        match self.len() {
            0 => Cow::Borrowed(""),
            1 => self[0].text_content(),
            _ => self
                .iter()
                .map(T::text_content)
                .collect::<Vec<_>>()
                .join("\n")
                .into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoteId(String);

impl Deref for NoteId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for NoteId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for NoteId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct TextSpan {
    pub text: String,
}

impl TextContent for TextSpan {
    fn text_content(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.text)
    }
}

#[derive(Clone, Debug)]
pub enum MarkupKind {
    Strong,
    Emphasis,
}

impl TryFrom<&pulldown_cmark::Tag<'_>> for MarkupKind {
    type Error = ();

    fn try_from(value: &pulldown_cmark::Tag<'_>) -> Result<Self, Self::Error> {
        match value {
            pulldown_cmark::Tag::Emphasis => Ok(Self::Emphasis),
            pulldown_cmark::Tag::Strong => Ok(Self::Strong),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MarkupSpan {
    pub kind: MarkupKind,
    pub inner: Vec<Span>,
}

impl TextContent for MarkupSpan {
    fn text_content(&self) -> Cow<'_, str> {
        self.inner.text_content()
    }
}

#[derive(Clone, Debug)]
pub enum Span {
    Text(TextSpan),
    Markup(MarkupSpan),
}

impl TextContent for Span {
    fn text_content(&self) -> Cow<'_, str> {
        match self {
            Self::Text(span) => span.text_content(),
            Self::Markup(annotation) => annotation.text_content(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Line {
    pub spans: Vec<Span>,
    pub child: Option<Box<Block>>,
}

impl TextContent for Line {
    fn text_content(&self) -> Cow<'_, str> {
        match (self.spans.is_empty(), &self.child) {
            (true, None) => Cow::Borrowed(""),
            (true, Some(child)) => child.text_content(),
            (false, None) => self.spans.text_content(),
            (false, Some(child)) => {
                Cow::Owned(self.spans.text_content().to_string() + "\n" + &child.text_content())
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Block {
    pub kind: BlockKind,
    pub items: Vec<Line>,
}

#[derive(Clone, Debug)]
pub enum BlockKind {
    Ol,
    Ul,
}

impl TryFrom<&'_ Tag<'_>> for BlockKind {
    type Error = ();

    fn try_from(value: &Tag) -> Result<Self, Self::Error> {
        match value {
            Tag::List(None) => Ok(Self::Ul),
            Tag::List(Some(_)) => Ok(Self::Ol),
            _ => Err(()),
        }
    }
}

impl TextContent for Block {
    fn text_content(&self) -> Cow<'_, str> {
        self.items.text_content()
    }
}

#[derive(Clone, Debug)]
pub struct Note {
    pub id: NoteId,
    pub metadata: Metadata,
    pub content: Vec<Block>,
}

impl Note {
    pub fn create(id: NoteId, text: &str) -> Result<Self, ()> {
        let front_matter_parser = gray_matter::Matter::<gray_matter::engine::YAML>::new();
        let front_matter = front_matter_parser.parse(text);

        let metadata = front_matter
            .data
            .and_then(|data| data.deserialize::<Metadata>().ok())
            .unwrap_or_default();

        let mut parser = pulldown_cmark::Parser::new(&front_matter.content);

        let mut content = vec![];

        while let Some(event) = parser.next() {
            let block = match event {
                Event::Start(tag) => {
                    if let Ok(kind) = BlockKind::try_from(&tag) {
                        Block {
                            kind,
                            items: take_lines_until(&mut parser, &tag),
                        }
                    } else {
                        panic!("unknown tag while parsing block: {tag:?}");
                    }
                }
                e => {
                    panic!("unparsed event: {e:?}");
                }
            };
            content.push(block);
        }

        Ok(Note {
            id,
            metadata,
            content,
        })
    }
}

fn take_spans(parser: &mut pulldown_cmark::Parser<'_, '_>, end_tag: &Tag) -> Vec<Span> {
    let mut spans = vec![];

    while let Some(e) = parser.next() {
        match e {
            Event::Text(text) => spans.push(Span::Text(TextSpan {
                text: text.into_string(),
            })),
            Event::Start(tag) => {
                if let Ok(kind) = MarkupKind::try_from(&tag) {
                    spans.push(Span::Markup(MarkupSpan {
                        kind,
                        inner: take_spans(parser, &tag),
                    }))
                } else {
                    eprintln!("unknown tag while parsing span: {tag:?}");
                }
            }
            Event::End(ending) if &ending == end_tag => {
                break;
            }
            _ => {
                eprintln!("unknown event while parsing span: {e:?}");
            }
        }
    }

    spans
}

fn take_lines_until(parser: &mut pulldown_cmark::Parser, end_tag: &Tag) -> Vec<Line> {
    let mut lines = vec![];

    while let Some(e) = parser.next() {
        match e {
            Event::Start(Tag::Item) => {
                lines.push(Line {
                    spans: take_spans(parser, &Tag::Item),
                    child: None,
                });
            }
            Event::End(ending) if &ending == end_tag => {
                break;
            }
            _ => {
                panic!("unknown event while parsing ul: {e:?}");
            }
        }
    }

    lines
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Metadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(with = "date_serde", default, skip_serializing_if = "Option::is_none")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(with = "date_serde", default, skip_serializing_if = "Option::is_none")]
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
}

mod date_serde {
    use serde::Deserialize;

    pub fn serialize<S>(
        date: &chrono::DateTime<chrono::Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&date.to_rfc2822())
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<chrono::DateTime<chrono::Utc>>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = if let Ok(s) = String::deserialize(deserializer) {
            s
        } else {
            return Ok(None);
        };

        Ok(Some(dateparser::parse(&s).map_err(|e| {
            <D::Error as serde::de::Error>::custom(format!("invalid date: {}", e))
        })?))
    }
}
