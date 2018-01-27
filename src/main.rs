#[macro_use]
extern crate clap;

extern crate walkdir;
extern crate id3;
extern crate indicatif;
extern crate filetime;
extern crate serde;
extern crate serde_json;
extern crate mp3_duration;

#[macro_use]
extern crate serde_derive;

mod track;
mod track_list;
mod folder;
mod folder_list;
mod music_library;
mod utils;

use clap::ArgMatches;
use std::env;
use std::path::Path;
use music_library::MusicLibrary;

fn main() {
    let args = get_args();

    let mut music_library = match args.value_of("METADATA") {
        Some(f) if file_exists(f) => MusicLibrary::new_from_file(f),
        _ => MusicLibrary::new(),
    };

    let music_dir = Path::new(args.value_of("LIBRARY").unwrap());
    let prev_dir = env::current_dir().unwrap();
    assert!(env::set_current_dir(music_dir).is_ok());
    match music_library.load_path(".") {
        Ok(_) => println!("Loaded path successfully: {:?}", &music_dir),
        Err(_) => println!("Error occured!"),
    }
    assert!(env::set_current_dir(prev_dir).is_ok());

    if let Some(path) = args.value_of("OUTPATH") {
        match music_library.write_to_path(path) {
            Ok(_) => println!("Wrote file to {:?}", path),
            Err(e) => println!("Errored writing file: {:?}", e),
        }
    }
}

fn file_exists(maybe_file: &str) -> bool {
    Path::new(maybe_file).exists()
}

fn get_args<'a>() -> ArgMatches<'a> {
    clap_app!(app =>
        (version: "1.0")
        (author: "Mukund <yaymukund@gmail.com>")
        (about: "Parses metadata from a directory of music")
        (@arg LIBRARY: -l --library +required +takes_value "Path to the music directory")
        (@arg METADATA: -m --metadata +takes_value "Path to the metadata file")
        (@arg OUTPATH: -o --output +takes_value "Path to output directory")
    ).get_matches()
}
