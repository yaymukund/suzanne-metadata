extern crate walkdir;
extern crate id3;
extern crate indicatif;

use std::env;
use indicatif::ProgressBar;

mod track;
mod music_library;

fn main() {
    let music_dir = env::args().nth(1)
        .expect("Please specify a path to the music dir");

    let mut i = 0;
    let music_library = music_library::MusicLibrary::new(music_dir);
    let progress_bar = ProgressBar::new(music_library.count() as u64);

    for mp3_path in music_library.mp3s() {
        i += 1;
        progress_bar.inc(1);
        track::Track::new_from_path(&mp3_path, i);
    }

    progress_bar.finish();
}
