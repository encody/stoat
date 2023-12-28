use std::borrow::Cow;

use super::{Line, TextContent};

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

impl TryFrom<&'_ pulldown_cmark::Tag<'_>> for BlockKind {
    type Error = ();

    fn try_from(value: &pulldown_cmark::Tag<'_>) -> Result<Self, Self::Error> {
        match value {
            pulldown_cmark::Tag::List(None) => Ok(Self::Ul),
            pulldown_cmark::Tag::List(Some(_)) => Ok(Self::Ol),
            _ => Err(()),
        }
    }
}

impl TextContent for Block {
    fn text_content(&self) -> Cow<'_, str> {
        self.items.text_content()
    }
}
