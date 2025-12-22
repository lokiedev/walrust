use std::path::PathBuf;

pub struct ImageFile {
    name: PathBuf,
    folder: PathBuf,
}

impl ImageFile {
    pub fn new(name: impl Into<PathBuf>, folder: impl Into<PathBuf>) -> Self {
        ImageFile {
            name: name.into(),
            folder: folder.into(),
        }
    }

    pub fn path(&self) -> PathBuf {
        self.folder.join(&self.name)
    }
}
