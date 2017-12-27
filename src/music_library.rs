use walkdir::{DirEntry, Error, WalkDir};

pub struct MusicLibrary {
    mp3s: Vec<String>,
}

impl MusicLibrary {
    pub fn new(path: String) -> MusicLibrary {
        let mp3s: Vec<String> = WalkDir::new(&path).into_iter()
            .filter_map(get_mp3)
            .collect();

        MusicLibrary {
            mp3s,
        }
    }

    pub fn count(&self) -> usize {
        self.mp3s.len()
    }

    pub fn mp3s(&self) -> &Vec<String> {
        &self.mp3s
    }
}

fn get_mp3(entry_result: Result<DirEntry, Error>) -> Option<String> {
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
