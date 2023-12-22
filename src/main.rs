use std::path::{Path, PathBuf};

use clap::Parser;
use pulldown_cmark::{Event, Tag};
use stoat::note::{Block, Line, Metadata, Note, NoteId, Span, TextSpan, Ul};

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

            fn parse_line(md_doc: &mut pulldown_cmark::Parser) -> Line {
                let mut line = Line {
                    spans: vec![],
                    child: None,
                };

                while let Some(e) = md_doc.next() {
                    match e {
                        Event::Text(text) => line.spans.push(Span::Text(TextSpan {
                            text: text.into_string(),
                        })),
                        Event::End(Tag::Item) => {
                            break;
                        }
                        _ => {
                            eprintln!("unknown event while parsing list item: {e:?}");
                        }
                    }
                }

                line
            }

            fn parse_ul(md_doc: &mut pulldown_cmark::Parser) -> Ul {
                let mut items = vec![];

                while let Some(e) = md_doc.next() {
                    match e {
                        Event::Start(Tag::Item) => {
                            items.push(parse_line(md_doc));
                        }
                        Event::End(Tag::List(None)) => {
                            break;
                        }
                        e => {
                            eprintln!("unknown event while parsing ul: {e:?}");
                        }
                    }
                }

                Ul::new(items)
            }

            while let Some(event) = md_doc.next() {
                match event {
                    Event::Start(Tag::List(None)) => {
                        parse_ul(&mut md_doc);
                    }
                    e => {
                        println!("unparsed event: {e:?}");
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
