use crate::cli_parser::FileArgs;
use m3u::Entry;
use std::fs::read_dir;
use std::path::PathBuf;

pub fn build_playlist(file_args: &FileArgs) -> Vec<Entry> {
    let mut playlist: Vec<Entry> = Vec::default();
    if file_args.files.is_some() {
        for file in file_args.files.as_ref().unwrap() {
            playlist.push(m3u::path_entry(file));
        }
    }
    if file_args.directories.is_some() {
        for directory in file_args.directories.as_ref().unwrap() {
            let mut files: Vec<PathBuf> = Vec::default();
            match read_dir(directory) {
                Ok(entries) => {
                    for entry in entries {
                        match entry {
                            Ok(entry) => {
                                let path = entry.path();
                                files.push(path);
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
            files.sort();
            for file in files {
                playlist.push(m3u::path_entry(file));
            }
        }
    }

    playlist
}
