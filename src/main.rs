#[macro_use]
extern crate clap;

extern crate walkdir;
extern crate id3;
extern crate indicatif;
extern crate filetime;
extern crate serde;
extern crate serde_json;

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

fn main() {
    let args = get_args();
    let mut music_library;

    if let Some(metadata_file) = args.value_of("METADATA") {
        music_library = music_library::MusicLibrary::new_from_file(metadata_file);
    } else {
        music_library = music_library::MusicLibrary::new();
    }

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
