use std::fs;
use walkdir::DirEntry;
use filetime::FileTime;
use utils::strip_currentdir;

#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    id: u32,
    created_at: u64,
    path: String,
}

impl Folder {
    pub fn new(id: u32, entry: &DirEntry) -> Folder {
        let metadata = fs::metadata(entry.path()).unwrap();
        let created_at = FileTime::from_last_modification_time(&metadata).seconds_relative_to_1970();
        let path = strip_currentdir(entry.path());

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
