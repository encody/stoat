use std::{borrow::Cow, ops::Deref};

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
    Ul(Vec<Line>),
    Ol(Vec<Line>),
    Code(String),
    Blockquote(Box<Block>),
}

impl TextContent for Block {
    fn text_content(&self) -> Cow<'_, str> {
        match self {
            Self::Ul(lines) | Self::Ol(lines) => lines.text_content(),
            Self::Code(code) => Cow::Borrowed(code),
            Self::Blockquote(block) => block.text_content(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Note {
    pub id: NoteId,
    pub title: String,
    pub content: Block,
}
