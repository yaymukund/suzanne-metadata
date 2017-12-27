use walkdir::{DirEntry, Error, IntoIter, WalkDir};
use std::iter::FilterMap;

pub struct MusicLibrary {
    path: String,
}

impl MusicLibrary {
    pub fn new(path: String) -> MusicLibrary {
        MusicLibrary { path }
    }

    pub fn count(&self) -> usize {
        self.mp3s().count()
    }

    pub fn mp3s(&self) -> FilterMap<IntoIter, fn(Result<DirEntry, Error>) -> Option<String>> {
        WalkDir::new(&self.path)
            .into_iter()
            .filter_map(mp3_path)
    }
}

fn mp3_path(entry_result: Result<DirEntry, Error>) -> Option<String> {
    match entry_result {
        Ok(ref e) if is_mp3(&e) => Some(entry_to_path(&e)),
        _ => None,
    }
}

fn is_mp3(entry: &DirEntry) -> bool {
    entry.path().extension()
        .and_then(|ext| ext.to_str()) == Some("mp3")
}

fn entry_to_path(entry: &DirEntry) -> String {
    entry.path().to_str().unwrap().to_string()
}
