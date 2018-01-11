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

use clap::ArgMatches;

mod track;
mod track_list;
mod folder;
mod folder_list;
mod music_library;

fn main() {
    let args = get_args();
    let music_dir = args.value_of("LIBRARY").unwrap();
    let mut music_library;

    if let Some(metadata_file) = args.value_of("METADATA") {
        music_library = music_library::MusicLibrary::new_from_file(metadata_file);
    } else {
        music_library = music_library::MusicLibrary::new();
    }

    match music_library.load_path(&music_dir) {
        Ok(_) => println!("Loaded path successfully: {}", &music_dir),
        Err(_) => println!("Error occured!"),
    }

    if let Some(path) = args.value_of("OUTPATH") {
        match music_library.write_to_path(path) {
            Ok(_) => println!("Wrote file to {}", path),
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
