use std::{borrow::Cow, ops::Deref};

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

impl TextContent for Block {
    fn text_content(&self) -> Cow<'_, str> {
        self.items.text_content()
    }
}

#[derive(Clone, Debug)]
pub struct Note {
    pub id: NoteId,
    pub metadata: Metadata,
    pub content: Block,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Metadata {
    pub title: Option<String>,
    #[serde(rename = "created")]
    pub created_ms: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "modified")]
    pub modified_ms: Option<chrono::DateTime<chrono::Utc>>,
}
