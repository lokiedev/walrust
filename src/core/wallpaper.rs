pub struct Wallpaper {
    pub name: String,
    pub path: String,
}

impl Wallpaper {
    pub fn new(name: String, path: String) -> Self {
        Wallpaper { name, path }
    }
}
