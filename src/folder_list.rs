use walkdir::DirEntry;
use folder::Folder;

pub struct FolderList {
    folders: Vec<Folder>,
    path: String,
}

impl FolderList {
    pub fn new(path: &str) -> FolderList {
        FolderList {
            path: path.to_string(),
            folders: Vec::new(),
        }
    }

    pub fn add_dir_entry(&mut self, entry: &DirEntry) -> &Folder {
        let folder_id = self.folders.len() as u32;
        let path = entry.path().to_path_buf();
        let folder = Folder::new(folder_id, path, &self.path);
        self.folders.push(folder);
        self.folders.last().unwrap()
    }
}
