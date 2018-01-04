use walkdir::{WalkDir, DirEntry};
use track_list::TrackList;
use folder_list::FolderList;
use indicatif::{ProgressBar, ProgressStyle};

pub struct MusicLibrary {
    path: String,
    tracks: TrackList,
    folders: FolderList,
}

impl MusicLibrary {
    pub fn new(path: String) -> MusicLibrary {
        MusicLibrary {
            tracks: TrackList::new(),
            folders: FolderList::new(&path),
            path,
        }
    }

    fn entries(&self) -> Vec<DirEntry> {
        WalkDir::new(&self.path)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .collect::<Vec<_>>()
    }

    fn load_entry(&mut self, entry: DirEntry) {
        let tracks = self.tracks.add_dir_entry(&entry);
        if !tracks.is_empty() {
            let folder = self.folders.add_dir_entry(&entry);
            let folder_id = folder.get_id();

            for track in tracks {
                track.set_folder_id(folder_id);
            }
        }
    }


    pub fn initialize(&mut self) {
        let entries = self.entries();
        let progress_bar = ProgressBar::new(entries.len() as u64);
        let style = ProgressStyle::default_bar()
            .template("{msg:20!} {wide_bar} {pos}/{len}");
        progress_bar.set_style(style);

        for entry in entries {
            progress_bar.set_message(entry.path().file_name().unwrap().to_str().unwrap());
            self.load_entry(entry);
            progress_bar.inc(1);
        }

        progress_bar.finish();
    }
}
