use std::fs;
use std::path::PathBuf;
use filetime::FileTime;

#[derive(Debug)]
pub struct Folder {
    id: u32,
    created_at: FileTime,
    path: String,
}

impl Folder {
    pub fn new(id: u32, path: PathBuf, library_path: &str) -> Folder {
        let metadata = fs::metadata(&path).unwrap();
        let relative_path = Folder::relative_path(path, library_path);
        let created_at = FileTime::from_last_modification_time(&metadata);

        Folder {
            id,
            path: relative_path,
            created_at,
        }
    }

    fn relative_path(path: PathBuf, library_path: &str) -> String {
        path.strip_prefix(library_path)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}
