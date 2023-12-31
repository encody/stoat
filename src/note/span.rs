use super::{markdown::Markdown, plain_text::PlainText, Render};

#[derive(Clone, Debug)]
pub struct TextSpan {
    pub text: String,
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

#[derive(Clone, Debug)]
pub enum Span {
    Text(TextSpan),
    Markup(MarkupSpan),
}

impl Render<PlainText> for TextSpan {
    fn render(&self) -> PlainText {
        PlainText(self.text.to_string())
    }
}

impl Render<PlainText> for MarkupSpan {
    fn render(&self) -> PlainText {
        self.inner.render()
    }
}

impl Render<PlainText> for Span {
    fn render(&self) -> PlainText {
        match self {
            Span::Text(t) => t.render(),
            Span::Markup(m) => m.render(),
        }
    }
}

impl Render<PlainText> for [Span] {
    fn render(&self) -> PlainText {
        PlainText(
            self.iter()
                .map(|s| Render::<PlainText>::render(s).0)
                .collect::<String>(),
        )
    }
}

impl Render<Markdown> for TextSpan {
    fn render(&self) -> Markdown {
        Markdown(self.text.to_string())
    }
}

impl Render<Markdown> for MarkupSpan {
    fn render(&self) -> Markdown {
        let rendered = Render::<Markdown>::render(&*self.inner);

        let delimiter = match self.kind {
            MarkupKind::Strong => "**",
            MarkupKind::Emphasis => "_",
        };

        Markdown([delimiter, &*rendered, delimiter].concat())
    }
}

impl Render<Markdown> for Span {
    fn render(&self) -> Markdown {
        match self {
            Span::Text(s) => s.render(),
            Span::Markup(s) => s.render(),
        }
    }
}

impl Render<Markdown> for [Span] {
    fn render(&self) -> Markdown {
        Markdown(
            self.iter()
                .map(|s| Render::<Markdown>::render(s).0)
                .collect::<String>(),
        )
    }
}
