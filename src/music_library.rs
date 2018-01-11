use walkdir::{WalkDir, DirEntry};
use track_list::TrackList;
use folder_list::FolderList;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json;
use std::io::Error;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
pub struct MusicLibrary {
    tracks: TrackList,
    folders: FolderList,
}

impl MusicLibrary {
    pub fn new() -> MusicLibrary {
        MusicLibrary {
            tracks: TrackList::new(),
            folders: FolderList::new(),
        }
    }

    pub fn new_from_file(metadata_file: &str) -> MusicLibrary {
        let file = File::open(metadata_file).unwrap();
        serde_json::from_reader(file).unwrap()
    }

    fn set_path(&mut self, path: String) {
        self.folders.set_path(path);
    }

    fn entries_in_path(&self, path: &str) -> Vec<DirEntry> {
        WalkDir::new(path)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .collect::<Vec<_>>()
    }

    fn load_entry(&mut self, entry: DirEntry) {
        if self.folders.has_entry(&entry) {
            return;
        }

        let tracks = self.tracks.add_dir_entry(&entry);
        if !tracks.is_empty() {
            let folder = self.folders.add_dir_entry(&entry);
            let folder_id = folder.get_id();

            for track in tracks {
                track.set_folder_id(folder_id);
            }
        }
    }

    pub fn load_path(&mut self, path: &str) -> Result<(), Error> {
        self.set_path(path.to_string());
        let entries = self.entries_in_path(path);
        let entries_count = entries.len() as u64;
        let initial_folders_count = self.folders.len();
        let progress_bar = ProgressBar::new(entries_count.clone());
        let style = ProgressStyle::default_bar()
            .template("{msg:20!} {wide_bar} {pos}/{len}");
        progress_bar.set_style(style);

        for entry in entries {
            progress_bar.set_message(entry.path().file_name().unwrap().to_str().unwrap());
            self.load_entry(entry);
            progress_bar.inc(1);
        }

        progress_bar.finish();

        println!("Processed {} folders in path", entries_count);
        println!("Found {} new folders", self.folders.len() - initial_folders_count);

        Ok(())
    }

    pub fn write_to_path(&self, path: &str) -> Result<(), Error> {
        let file = File::create(path)?;
        serde_json::to_writer(file, &self)?;
        Ok(())
    }
}
