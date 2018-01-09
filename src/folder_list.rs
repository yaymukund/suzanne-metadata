use walkdir::DirEntry;
use folder::Folder;
use serde::ser::{Serialize, Serializer};
use std::fmt;
use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};

#[derive(Debug)]
pub struct FolderList {
    folders: Vec<Folder>,
    path: Option<String>,
}

impl FolderList {
    pub fn new(path: &str) -> FolderList {
        FolderList {
            path: Some(path.to_string()),
            folders: Vec::new(),
        }
    }

    pub fn without_path() -> FolderList {
        FolderList {
            path: None,
            folders: Vec::new(),
        }
    }

    pub fn push(&mut self, folder: Folder) -> &Folder {
        self.folders.push(folder);
        self.folders.last().unwrap()
    }

    pub fn add_dir_entry(&mut self, entry: &DirEntry) -> &Folder {
        let folder_id = self.folders.len() as u32;
        let folder = Folder::new(folder_id, entry, &self);
        self.folders.push(folder);
        self.folders.last().unwrap()
    }

    pub fn relative_path_to(&self, entry: &DirEntry) -> String {
        let path = self.path.as_ref().expect(
            "can't call folder_list.relative_path_to() before setting folder_list.path"
        );

        entry.path()
            .strip_prefix(path)
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

struct FolderListVisitor;

impl<'de> Visitor<'de> for FolderListVisitor {
    type Value = FolderList;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a list of folders")
    }

    fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
            where S: SeqAccess<'de>
    {
        let mut folder_list = FolderList::without_path();
        while let Some(folder) = seq.next_element()? {
            folder_list.push(folder);
        }

        Ok(folder_list)
    }
}

impl<'de> Deserialize<'de> for FolderList {
    fn deserialize<D>(deserializer: D) -> Result<FolderList, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_seq(FolderListVisitor)
    }
}

impl Serialize for FolderList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        self.folders.serialize(serializer)
    }
}
