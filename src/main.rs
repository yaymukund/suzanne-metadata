#[macro_use]

extern crate clap;
extern crate walkdir;
extern crate id3;
extern crate indicatif;
extern crate filetime;

use clap::ArgMatches;

mod track;
mod track_list;
mod folder;
mod folder_list;
mod music_library;

fn main() {
    let args = get_args();
    let music_dir = args.value_of("LIBRARY").unwrap();
    let mut music_library = music_library::MusicLibrary::new(music_dir.to_string());
    music_library.initialize();
}

fn get_args<'a>() -> ArgMatches<'a> {
    clap_app!(app =>
        (version: "1.0")
        (author: "Mukund <yaymukund@gmail.com>")
        (about: "Parses metadata from a directory of music")
        (@arg LIBRARY: -l --library +required +takes_value "Path to the music directory")
        (@arg METADATA: -m --metadata +takes_value "Path to the metadata file")
        (@arg INDEX: -i --index +takes_value "Path to the index metadata file")
        (@arg OUTPATH: -o --output +takes_value "Path to output directory")
    ).get_matches()
}
