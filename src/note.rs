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
pub enum AnnotationSpan {
    Bold(Vec<Span>),
    Italic(Vec<Span>),
    Underline(Vec<Span>),
    Strikeout(Vec<Span>),
    Code(Vec<Span>),
    Link { span: Vec<Span>, path: String },
    Tag(Vec<Span>),
}

impl TextContent for AnnotationSpan {
    fn text_content(&self) -> Cow<'_, str> {
        match self {
            Self::Bold(span)
            | Self::Italic(span)
            | Self::Underline(span)
            | Self::Strikeout(span)
            | Self::Code(span)
            | Self::Link { span, .. }
            | Self::Tag(span) => span.text_content(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Span {
    Text(TextSpan),
    Annotation(Box<AnnotationSpan>),
}

impl TextContent for Span {
    fn text_content(&self) -> Cow<'_, str> {
        match self {
            Self::Text(span) => span.text_content(),
            Self::Annotation(annotation) => annotation.text_content(),
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
pub enum Block {
    Header(Header),
    Ul(Ul),
    Ol(Ol),
    Code(Code),
    Blockquote(Blockquote),
}

impl TextContent for Block {
    fn text_content(&self) -> Cow<'_, str> {
        match self {
            Self::Header(header) => header.text_content(),
            Self::Ul(ul) => ul.text_content(),
            Self::Ol(ol) => ol.text_content(),
            Self::Code(code) => code.text_content(),
            Self::Blockquote(block) => block.text_content(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Header {
    pub level: u8,
    pub text: Vec<Span>,
}

impl TextContent for Header {
    fn text_content(&self) -> Cow<'_, str> {
        self.text.text_content()
    }
}

#[derive(Clone, Debug)]
pub struct Ul {
    pub items: Vec<Line>,
}

impl Ul {
    pub fn new(items: Vec<Line>) -> Self {
        Self { items }
    }
}

impl TextContent for Ul {
    fn text_content(&self) -> Cow<'_, str> {
        self.items.text_content()
    }
}

#[derive(Clone, Debug)]
pub struct Ol {
    pub items: Vec<Line>,
}

impl Ol {
    pub fn new(items: Vec<Line>) -> Self {
        Self { items }
    }
}

impl TextContent for Ol {
    fn text_content(&self) -> Cow<'_, str> {
        self.items.text_content()
    }
}

#[derive(Clone, Debug)]
pub struct Code {
    pub code: String,
}

impl TextContent for Code {
    fn text_content(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.code)
    }
}

#[derive(Clone, Debug)]
pub struct Blockquote {
    pub content: Vec<Block>,
}

impl TextContent for Blockquote {
    fn text_content(&self) -> Cow<'_, str> {
        self.content.text_content()
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
