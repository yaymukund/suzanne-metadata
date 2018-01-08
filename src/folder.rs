use std::fs;
use walkdir::DirEntry;
use filetime::FileTime;
use folder_list::FolderList;

#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    id: u32,
    created_at: u64,
    path: String,
}

impl Folder {
    pub fn new(id: u32, entry: &DirEntry, folder_list: &FolderList) -> Folder {
        let metadata = fs::metadata(entry.path()).unwrap();
        let created_at = FileTime::from_last_modification_time(&metadata).seconds_relative_to_1970();
        let path = folder_list.relative_path_to(entry);

        Folder {
            id,
            path,
            created_at,
        }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}
