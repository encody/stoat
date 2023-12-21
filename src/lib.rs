use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoteId(String);

pub struct Note {
    pub id: NoteId,
    pub title: String,
    pub content: Vec<ContentNode>,
}

pub enum Span {
    Text(TextSpan),
    Annotation(Box<AnnotationSpan>),
}

pub struct TextSpan {
    pub text: String,
}

pub enum AnnotationSpan {
    Bold(Span),
    Italic(Span),
    Underline(Span),
    Strikeout(Span),
    Code(Span),
    Link {
        span: Span,
        path: String,
    },
    Tag(Span),
}

pub struct TextItem {
    pub spans: Vec<Span>,
}

pub struct ContentNode {
    pub content: String,
    pub children: Vec<ContentNode>,
}

pub struct Notebook {
    pub store: HashMap<NoteId, Note>,
}

impl Notebook {}

#[cfg(test)]
mod tests {}
