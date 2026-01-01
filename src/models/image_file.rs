use std::{
    fmt::{self, Display},
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub struct ImageFile {
    pub name: String,
    pub path: PathBuf,
}

impl ImageFile {
    pub fn new(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        ImageFile {
            name: name.into(),
            path: path.into(),
        }
    }
}

impl Display for ImageFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.name)
    }
}
