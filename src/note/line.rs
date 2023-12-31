use super::{markdown::Markdown, plain_text::PlainText, Block, Render, Span};

#[derive(Clone, Debug)]
pub struct Line {
    pub spans: Vec<Span>,
    pub child: Option<Box<Block>>,
}

impl Render<PlainText> for Line {
    fn render(&self) -> PlainText {
        self.spans.render()
    }
}

impl Render<Markdown> for Line {
    fn render(&self) -> Markdown {
        self.spans.render()
    }
}
