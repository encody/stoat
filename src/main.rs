use std::path::{Path, PathBuf};

use clap::Parser;
use pulldown_cmark::{Event, Tag};
use stoat::note::{Block, BlockKind, Line, MarkupKind, Metadata, Note, NoteId, Span, TextSpan};

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long)]
    index: PathBuf,
}

fn load_directory(path: &Path) -> Vec<Note> {
    let files_in_index = std::fs::read_dir(path).unwrap();

    let front_matter_parser = gray_matter::Matter::<gray_matter::engine::YAML>::new();

    files_in_index
        .flatten()
        .filter_map(|file| {
            let file_name = file.file_name().into_string().ok()?;

            // ignore files with leading . or _
            // ignore non-markdown files
            if file_name.starts_with(['.', '_']) || !file_name.ends_with(".md") {
                return None;
            }

            // .metadata() traverses symlinks
            let metadata = file.metadata().ok()?;

            if !metadata.is_file() {
                return None;
            }

            println!("{}", file.path().display());

            let contents = std::fs::read_to_string(file.path()).ok()?;

            println!("{}", contents);

            let front_matter = front_matter_parser.parse(&contents);

            let metadata: Metadata = front_matter.data.unwrap().deserialize().unwrap();

            println!("Metadata:");

            println!("{:?}", metadata);

            let id: NoteId = file_name[..file_name.len() - 3].into();

            let mut md_doc = pulldown_cmark::Parser::new(&front_matter.content);

            fn take_spans(md_doc: &mut pulldown_cmark::Parser<'_, '_>, end_tag: &Tag) -> Vec<Span> {
                let mut spans = vec![];

                while let Some(e) = md_doc.next() {
                    match e {
                        Event::Text(text) => spans.push(Span::Text(TextSpan {
                            text: text.into_string(),
                        })),
                        Event::Start(tag) => {
                            if let Ok(kind) = MarkupKind::try_from(&tag) {
                                spans.push(Span::Markup(stoat::note::MarkupSpan {
                                    kind,
                                    inner: take_spans(md_doc, &tag),
                                }))
                            } else {
                                eprintln!("unknown tag while parsing span: {tag:?}");
                            }
                        }
                        Event::End(ending) if &ending == end_tag => {
                            break;
                        }
                        _ => {
                            eprintln!("unknown event while parsing span: {e:?}");
                        }
                    }
                }

                spans
            }

            fn take_item(md_doc: &mut pulldown_cmark::Parser) -> Line {
                Line {
                    spans: take_spans(md_doc, &Tag::Item),
                    child: None,
                }
            }

            fn take_lines_until(md_doc: &mut pulldown_cmark::Parser, until: Tag) -> Vec<Line> {
                let mut lines = vec![];

                while let Some(e) = md_doc.next() {
                    match e {
                        Event::Start(Tag::Item) => {
                            lines.push(take_item(md_doc));
                        }
                        Event::End(ending) if ending == until => {
                            break;
                        }
                        e => {
                            panic!("unknown event while parsing ul: {e:?}");
                        }
                    }
                }

                lines
            }

            while let Some(event) = md_doc.next() {
                match event {
                    Event::Start(Tag::List(None)) => {
                        let items = take_lines_until(&mut md_doc, Tag::List(None));
                        let block = Block {
                            kind: BlockKind::Ul,
                            items,
                        };
                    }
                    e => {
                        panic!("unparsed event: {e:?}");
                    }
                }
            }

            // let note = Note {
            //     id,
            //     metadata,
            //     content: todo!(),
            // };

            None

            // todo!()
        })
        .collect()
}

fn main() {
    let args = Args::parse();

    load_directory(&args.index);

    // let project_dirs = directories::ProjectDirs::from("", "", "stoat").unwrap();
}
