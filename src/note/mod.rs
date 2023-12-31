use std::ops::Deref;

use pulldown_cmark::{Event, Tag};

mod block;
pub use block::*;
mod line;
pub use line::*;
mod metadata;
pub use metadata::*;
mod render;
pub use render::*;
mod span;
pub use span::*;

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
pub struct Note {
    pub id: NoteId,
    pub metadata: Metadata,
    pub content: Vec<Block>,
}

impl Note {
    pub fn create(id: NoteId, text: &str) -> Result<Self, ()> {
        let front_matter_parser = gray_matter::Matter::<gray_matter::engine::YAML>::new();
        let front_matter = front_matter_parser.parse(text);

        // TODO: Preserve existing, unread attributes in metadata
        let metadata = front_matter
            .data
            .and_then(|data| data.deserialize::<Metadata>().ok())
            .unwrap_or_default();

        let mut parser = pulldown_cmark::Parser::new(&front_matter.content);

        let mut content = vec![];

        while let Some(event) = parser.next() {
            let block = match event {
                pulldown_cmark::Event::Start(tag) => {
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
            pulldown_cmark::Event::Text(text) => spans.push(Span::Text(TextSpan {
                text: text.into_string(),
            })),
            pulldown_cmark::Event::Start(tag) => {
                if let Ok(kind) = MarkupKind::try_from(&tag) {
                    spans.push(Span::Markup(MarkupSpan {
                        kind,
                        inner: take_spans(parser, &tag),
                    }))
                } else {
                    eprintln!("unknown tag while parsing span: {tag:?}");
                }
            }
            pulldown_cmark::Event::End(ending) if &ending == end_tag => {
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
