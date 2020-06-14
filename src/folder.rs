use filetime::FileTime;
use std::convert::TryInto;
use std::fs;
use utils::strip_currentdir;
use walkdir::DirEntry;

#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    id: u32,
    created_at: u64,
    path: String,
}

impl Folder {
    pub fn new(id: u32, entry: &DirEntry) -> Folder {
        let metadata = fs::metadata(entry.path()).unwrap();
        let created_at = FileTime::from_last_modification_time(&metadata).unix_seconds();
        let created_at = created_at.try_into().unwrap();
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
