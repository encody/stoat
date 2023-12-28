use std::ops::Deref;

use super::{Block, BlockKind, Line, MarkupKind, MarkupSpan, Span, TextSpan};

pub trait Render<T> {
    fn render(self) -> T;
}

pub struct Markdown(String);

impl Deref for Markdown {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Render<Markdown> for TextSpan {
    fn render(self) -> Markdown {
        Markdown(self.text)
    }
}

impl Render<Markdown> for MarkupSpan {
    fn render(self) -> Markdown {
        match self.kind {
            MarkupKind::Strong => Markdown(format!("**{}**", &*self.inner.render())),
            MarkupKind::Emphasis => Markdown(format!("_{}_", &*self.inner.render())),
        }
    }
}

impl Render<Markdown> for Span {
    fn render(self) -> Markdown {
        match self {
            Span::Text(s) => s.render(),
            Span::Markup(s) => s.render(),
        }
    }
}

impl Render<Markdown> for Vec<Span> {
    fn render(self) -> Markdown {
        Markdown(self.into_iter().map(|s| s.render().0).collect::<String>())
    }
}

impl Render<Markdown> for Line {
    fn render(self) -> Markdown {
        Markdown(self.spans.render().0)
    }
}

impl Render<Markdown> for Block {
    fn render(self) -> Markdown {
        match self.kind {
            BlockKind::Ol => Markdown(
                self.items
                    .into_iter()
                    .enumerate()
                    .map(|(i, l)| format!("{}. {}", i + 1, l.render().0))
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
            BlockKind::Ul => Markdown(
                self.items
                    .into_iter()
                    .map(|l| format!("- {}", l.render().0))
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
        }
    }
}

impl Render<Markdown> for Vec<Block> {
    fn render(self) -> Markdown {
        Markdown(
            self.into_iter()
                .map(|b| b.render().0)
                .collect::<Vec<_>>()
                .join("\n\n"),
        )
    }
}
