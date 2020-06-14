use folder::Folder;
use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;
use utils::strip_currentdir;
use walkdir::DirEntry;

#[derive(Debug)]
pub struct FolderList {
    folders: Vec<Folder>,
}

impl FolderList {
    pub fn new() -> FolderList {
        FolderList {
            folders: Vec::new(),
        }
    }

    pub fn push(&mut self, folder: Folder) -> &Folder {
        self.folders.push(folder);
        self.folders.last().unwrap()
    }

    pub fn add_dir_entry(&mut self, entry: &DirEntry) -> &Folder {
        let folder_id = self.folders.len() as u32;
        let folder = Folder::new(folder_id, entry);
        self.folders.push(folder);
        self.folders.last().unwrap()
    }

    pub fn has_entry(&self, entry: &DirEntry) -> bool {
        let path = strip_currentdir(entry.path());

        self.folders
            .iter()
            .any(|folder| *folder.get_path() == *path)
    }

    pub fn len(&self) -> usize {
        self.folders.len()
    }
}

struct FolderListVisitor;

impl<'de> Visitor<'de> for FolderListVisitor {
    type Value = FolderList;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a list of folders")
    }

    fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
    where
        S: SeqAccess<'de>,
    {
        let mut folder_list = FolderList::new();
        while let Some(folder) = seq.next_element()? {
            folder_list.push(folder);
        }

        Ok(folder_list)
    }
}

impl<'de> Deserialize<'de> for FolderList {
    fn deserialize<D>(deserializer: D) -> Result<FolderList, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(FolderListVisitor)
    }
}

impl Serialize for FolderList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.folders.serialize(serializer)
    }
}
