use track::Track;
use walkdir::{DirEntry, Error, WalkDir};

pub struct TrackList {
    tracks: Vec<Track>,
}

impl TrackList {
    pub fn new() -> TrackList {
        TrackList {
            tracks: Vec::new(),
        }
    }

    pub fn add_dir_entry(&mut self, entry: &DirEntry) -> &mut[Track] {
        if is_mp3(entry) {
            self.add_track_entry(entry)
        } else if is_dir(entry) {
            self.add_folder_entry(entry)
        } else {
            self.empty_slice()
        }
    }

    fn add_track_entry(&mut self, entry: &DirEntry) -> &mut[Track] {
        let result = self.add_track(entry);
        if result.is_ok() {
            self.last(1)
        } else {
            self.empty_slice()
        }
    }

    fn add_folder_entry(&mut self, entry: &DirEntry) -> &mut[Track] {
        let entries = WalkDir::new(entry.path())
            .into_iter()
            .filter_map(mp3_path);
        let mut count = 0;

        for entry in entries {
            if self.add_track(&entry).is_ok() {
                count += 1;
            }
        }

        self.last(count)
    }

    fn add_track(&mut self, entry: &DirEntry) -> Result<(), ()> {
        let id = self.tracks.len() as u32;
        let path = entry.path().to_path_buf();

        let track = Track::new_from_path(path, id).ok_or(())?;
        self.tracks.push(track);
        Ok(())
    }

    fn last(&mut self, count: usize) -> &mut[Track] {
        let len = self.tracks.len();
        &mut self.tracks[len-count..]
    }

    fn empty_slice(&mut self) -> &mut[Track] {
        self.last(0)
    }
}

fn mp3_path(entry_result: Result<DirEntry, Error>) -> Option<DirEntry> {
    let filter_mp3 = |entry| if is_mp3(&entry) { Some(entry) } else { None };
    entry_result.ok().and_then(filter_mp3)
}

fn is_dir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

fn is_mp3(entry: &DirEntry) -> bool {
    entry.path().extension()
        .and_then(|ext| ext.to_str()) == Some("mp3")
}
