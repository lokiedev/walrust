use std::path::PathBuf;

pub struct ImageFile {
    name: String,
    path: PathBuf,
}

impl ImageFile {
    pub fn new(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        ImageFile {
            name: name.into(),
            path: path.into(),
        }
    }
}
