use std::path::{Path, PathBuf};

use clap::Parser;
use stoat::note::{Note, NoteId};

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long)]
    index: PathBuf,
}

fn load_directory(path: &Path) -> Vec<Note> {
    let files_in_index = std::fs::read_dir(path).unwrap();

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
            let id: NoteId = file_name[..file_name.len() - 3].into();

            Note::create(id, &contents).ok()
        })
        .collect()
}

fn main() {
    let args = Args::parse();

    let notes = load_directory(&args.index);

    println!("Notes:\n\n{notes:#?}");

    // let project_dirs = directories::ProjectDirs::from("", "", "stoat").unwrap();
}
