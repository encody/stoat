use std::path::{Path, PathBuf};

use clap::Parser;
use stoat::note::{markdown::Markdown, Note, NoteId, Render};

static IGNORE_FILES_WITH_LEADING: &[char] = &['.', '_'];
static NOTES_FILE_EXTENSION: &str = "md"; // not including dot

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long)]
    index: PathBuf,
}

fn load_directory(path: &Path) -> Vec<Note> {
    let files_in_index = std::fs::read_dir(path).unwrap();

    files_in_index
        .flatten()
        .filter_map(|entry| try_load_file(&entry.path()))
        .collect()
}

fn try_load_file(path: &Path) -> Option<Note> {
    // .metadata() traverses symlinks
    let metadata = path.metadata().ok()?;

    if !metadata.is_file() {
        return None;
    }

    let file_name = path.file_name()?.to_str()?;

    let extension = path.extension()?;

    if file_name.starts_with(IGNORE_FILES_WITH_LEADING) || extension != NOTES_FILE_EXTENSION {
        return None;
    }

    let contents = std::fs::read_to_string(path).ok()?;

    let id: NoteId = file_name[..file_name.len() - (NOTES_FILE_EXTENSION.len() + 1)].into();

    Note::create(id, &contents).ok()
}

fn main() {
    let args = Args::parse();

    let notes = if args.index.is_dir() {
        load_directory(&args.index)
    } else {
        vec![try_load_file(&args.index).expect("Could not load file")]
    };

    // println!("Notes:\n\n{notes:#?}");

    for note in notes {
        println!("{}", &*Render::<Markdown>::render(&*note.content));
    }

    // let project_dirs = directories::ProjectDirs::from("", "", "stoat").unwrap();
}
