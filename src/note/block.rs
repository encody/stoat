use super::{markdown::Markdown, plain_text::PlainText, Line, Render};

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

impl Render<PlainText> for Block {
    fn render(&self) -> PlainText {
        match self.kind {
            BlockKind::Ol => PlainText(
                self.items
                    .iter()
                    .enumerate()
                    .map(|(i, l)| format!("{}. {}", i + 1, &*Render::<PlainText>::render(l)))
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
            BlockKind::Ul => PlainText(
                self.items
                    .iter()
                    .map(|l| format!("* {}", &*Render::<PlainText>::render(l)))
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
        }
    }
}

impl Render<PlainText> for [Block] {
    fn render(&self) -> PlainText {
        PlainText(
            self.iter()
                .map(|b| Render::<PlainText>::render(b).0)
                .collect::<Vec<_>>()
                .join("\n\n"),
        )
    }
}

impl Render<Markdown> for Block {
    fn render(&self) -> Markdown {
        match self.kind {
            BlockKind::Ol => Markdown(
                self.items
                    .iter()
                    .enumerate()
                    .map(|(i, l)| format!("{}. {}", i + 1, Render::<Markdown>::render(l).0))
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
            BlockKind::Ul => Markdown(
                self.items
                    .iter()
                    .map(|l| format!("- {}", Render::<Markdown>::render(l).0))
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
        }
    }
}

impl Render<Markdown> for [Block] {
    fn render(&self) -> Markdown {
        Markdown(
            self.iter()
                .map(|b| Render::<Markdown>::render(b).0)
                .collect::<Vec<_>>()
                .join("\n\n"),
        )
    }
}
