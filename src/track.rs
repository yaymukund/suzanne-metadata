use id3::{self, Error, ErrorKind, Tag};
use mp3_duration;
use std::fs::File;
use std::path::PathBuf;
use utils::strip_currentdir;

const DEFAULT_TAG: &str = "?";

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
    duration: String,
    path: String,
    folder_id: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackMetadata {
    id: u32,
    search: String,
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
            duration: duration(&tag, &path),
            path: strip_currentdir(&path),
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
                partial_tag: None,
            });
        }

        Ok(Track::new_from_tag(&tag, path, id))
    }

    pub fn set_folder_id(&mut self, id: u32) {
        self.folder_id = Some(id);
    }

    pub fn metadata(&self) -> TrackMetadata {
        TrackMetadata {
            id: self.id,
            search: format!("{}|{}|{}", self.title, self.album, self.artist),
        }
    }
}

fn title(tag: &Tag) -> String {
    tag.title().map_or_else(unknown_tag, String::from)
}

fn album(tag: &Tag) -> String {
    tag.album().map_or_else(unknown_tag, String::from)
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

fn duration(tag: &Tag, path: &PathBuf) -> String {
    tag.duration()
        .map(|d| (d + 999) / 1000) // Convert ms to s, rounding up
        .or_else(|| read_duration_from_file(&path))
        .map_or_else(unknown_tag, |d| d.to_string())
}

fn track_number(tag: &Tag) -> String {
    let track = tag.track().map_or_else(unknown_tag, |t| t.to_string());

    if let Some(disc) = tag.disc() {
        format!("{}.{}", disc, track)
    } else {
        track
    }
}

fn read_duration_from_file(path: &PathBuf) -> Option<u32> {
    mp3_duration::from_path(&path)
        .map(|d| d.as_secs() as u32)
        .ok()
}
