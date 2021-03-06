#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;

extern crate env_logger;
extern crate filetime;
extern crate id3;
extern crate indicatif;
extern crate mp3_duration;
extern crate serde;
extern crate serde_json;
extern crate walkdir;

#[macro_use]
extern crate serde_derive;

mod folder;
mod folder_list;
mod music_library;
mod track;
mod track_list;
mod utils;

use clap::ArgMatches;
use music_library::MusicLibrary;
use std::env;
use std::path::Path;

fn main() {
    env_logger::init();
    let args = get_args();

    let mut music_library = match args.value_of("METADATA") {
        Some(f) if file_exists(f) => MusicLibrary::new_from_file(f),
        _ => MusicLibrary::new(),
    };

    let music_dir = Path::new(args.value_of("LIBRARY").unwrap());
    let prev_dir = env::current_dir().unwrap();
    assert!(env::set_current_dir(music_dir).is_ok());
    match music_library.load_path(".") {
        Ok(_) => debug!("Loaded path successfully: {:?}", &music_dir),
        Err(_) => debug!("Error occured!"),
    }
    assert!(env::set_current_dir(prev_dir).is_ok());

    if let Some(path) = args.value_of("OUTPATH") {
        match music_library.write_to_path(path) {
            Ok(_) => debug!("Wrote file to {:?}", path),
            Err(e) => debug!("Errored writing file: {:?}", e),
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
    )
    .get_matches()
}
