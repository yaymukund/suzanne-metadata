use id3::{self, Tag, Error, ErrorKind};
use std::path::PathBuf;
use std::fs::File;

const DEFAULT_TAG: &'static str = "?";

fn unknown_tag() -> String {
    DEFAULT_TAG.to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    id: u32,
    title: String,
    album: String,
    artist: String,
    date: String,
    track_number: String,
    path: String,
    folder_id: Option<u32>,
}

impl Track {
    pub fn new_from_tag(tag: &Tag, path: PathBuf, id: u32) -> Track {
        Track {
            id,
            title: title(&tag),
            album: album(&tag),
            artist: artist(&tag),
            date: date(&tag),
            track_number: track_number(&tag),
            path: path.to_str().unwrap().to_string(),
            folder_id: None,
        }
    }

    pub fn new_from_path(path: PathBuf, id: u32) -> Result<Track, Error> {
        let mut file = File::open(&path)?;

        let tag;

        if Tag::is_candidate(&mut file)? {
            tag = Tag::read_from(file)?;
        } else if id3::v1::Tag::is_candidate(&mut file)? {
            tag = id3::v1::Tag::read_from(file)?.into();
        } else {
            return Err(Error {
                description: "File doesn't contain id3v1 or id3v2 tags.",
                kind: ErrorKind::NoTag,
            });
        }

        Ok(Track::new_from_tag(&tag, path, id))
    }

    pub fn set_folder_id(&mut self, id: u32) {
        self.folder_id = Some(id.clone());
    }
}

fn title(tag: &Tag) -> String {
    tag.title()
        .map_or_else(unknown_tag, String::from)
}

fn album(tag: &Tag) -> String {
    tag.album()
        .map_or_else(unknown_tag, String::from)
}

fn artist(tag: &Tag) -> String {
    tag.artist()
        .or_else(|| tag.album_artist())
        .map_or_else(unknown_tag, String::from)
}

fn date(tag: &Tag) -> String {
    tag.year()
        .or_else(|| tag.date_released().map(|d| d.year))
        .or_else(|| tag.date_recorded().map(|d| d.year))
        .map_or_else(unknown_tag, |d| d.to_string())
}

fn track_number(tag: &Tag) -> String {
    let track = tag.track()
        .map_or_else(unknown_tag, |t| t.to_string());

    if let Some(total) = tag.total_tracks() {
        format!("{}/{}", track, total)
    } else {
        track
    }
}
