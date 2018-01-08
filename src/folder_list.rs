use walkdir::DirEntry;
use folder::Folder;
use serde::ser::{Serialize, Serializer};

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
        let folder = Folder::new(folder_id, entry, &self);
        self.folders.push(folder);
        self.folders.last().unwrap()
    }

    pub fn relative_path_to(&self, entry: &DirEntry) -> String {
        entry.path()
            .strip_prefix(&self.path)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn has_entry(&self, entry: &DirEntry) -> bool {
        let path: String = self.relative_path_to(entry);
        self.folders.iter()
            .any(|folder| folder.get_path() == path)
    }
}

impl Serialize for FolderList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        self.folders.serialize(serializer)
    }
}
