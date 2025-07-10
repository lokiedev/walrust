#[derive(Debug, Clone)]
pub struct Wallpaper {
    pub name: String,
    pub path: String,
}

impl Wallpaper {
    pub fn new(name: String, path: String) -> Self {
        Self { name, path }
    }
}
