extern crate walkdir;
extern crate id3;

use std::env;
use walkdir::{DirEntry, WalkDir};

mod track;

fn is_mp3(entry: &DirEntry) -> bool {
    entry.path().extension().and_then(|ext| ext.to_str()) == Some("mp3")
}

fn main() {
    let music_dir = env::args().nth(1)
        .expect("Please specify a path to the music dir");

    let mut i = 0;
    for entry in WalkDir::new(music_dir) {
        i += 1;
        let entry = entry.unwrap();
        if is_mp3(&entry) {
            let path = entry.path().to_str().unwrap();
            match track::create_from_path(path, i) {
                Some(t) => println!("{:?}", t),
                None => println!("Errored on {}", path),
            }
        }
    }
}
