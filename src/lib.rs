use std::collections::HashMap;

pub mod note;

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
        println!("{}", chrono::Utc::now());
        let note = Note {
            id: "1".into(),
            metadata: Metadata {
                title: Some("title".to_string()),
                created: None,
                modified: None,
            },
            content: vec![Block {
                kind: BlockKind::Ul,
                items: vec![Line {
                    spans: vec![Span::Text(TextSpan {
                        text: "text".into(),
                    })],
                    child: None,
                }],
            }],
        };

        let content = note.content.text_content();

        assert_eq!(content, "text");
    }
}
