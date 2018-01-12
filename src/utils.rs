use std::path::Path;

pub fn strip_currentdir(path: &Path) -> String {
    path.strip_prefix("./")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}
