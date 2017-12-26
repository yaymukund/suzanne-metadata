use id3::Tag;

const DEFAULT_TAG: &'static str = "?";

#[derive(Debug)]
pub struct Track {
    id: u32,
    title: String,
    album: String,
    artist: String,
    date: String,
    track_number: String,
    path: String,
}

fn get_default() -> String {
    DEFAULT_TAG.to_string()
}

fn get_title(tag: &Tag) -> String {
    tag.title()
        .map_or_else(get_default, String::from)
}

fn get_album(tag: &Tag) -> String {
    tag.album()
        .map_or_else(get_default, String::from)
}

fn get_artist(tag: &Tag) -> String {
    tag.artist()
        .or_else(|| tag.album_artist())
        .map_or_else(get_default, String::from)
}

fn get_date(tag: &Tag) -> String {
    tag.year()
        .or_else(|| tag.date_released().map(|d| d.year))
        .or_else(|| tag.date_recorded().map(|d| d.year))
        .map_or_else(get_default, |d| d.to_string())
}

fn get_track_number(tag: &Tag) -> String {
    tag.track()
        .map_or_else(get_default, |t| t.to_string())
}

pub fn create_from_path(path: &str, id: u32) -> Option<Track> {
    let tag = match Tag::read_from_path(path) {
        Ok(t) => t,
        Err(_) => return None,
    };

    Some(Track {
        id,
        title: get_title(&tag),
        album: get_album(&tag),
        artist: get_artist(&tag),
        date: get_date(&tag),
        track_number: get_track_number(&tag),
        path: path.to_string(),
    })
}
