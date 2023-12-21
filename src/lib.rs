use std::collections::HashMap;

mod note;

#[derive(Default, Clone, Debug)]
pub struct Notebook {
    pub store: HashMap<note::NoteId, note::Note>,
}

impl Notebook {}

#[cfg(test)]
mod tests {
    use super::*;
    use note::*;

    #[test]
    fn test() {
        let note = Note {
            id: "1".into(),
            title: "title".into(),
            content: Block::Ul(vec![Line {
                spans: vec![Span::Text(TextSpan {
                    text: "text".into(),
                })],
                child: None,
            }]),
        };

        let content = note.content.text_content();

        assert_eq!(content, "text");
    }
}
