use std::env;

pub enum Desktop {
    Hyprland,
    Unknown,
}

impl Desktop {
    pub fn from(wm_name: &str) -> Self {
        match wm_name {
            "Hyprland" => Desktop::Hyprland,
            _ => Desktop::Unknown,
        }
    }

    pub fn from_env(env_var: &str) -> Self {
        if let Ok(wm_name) = env::var(env_var) {
            return Desktop::from(&wm_name);
        }
        Desktop::Unknown
    }

    pub fn detect() -> Self {
        let desktop = Desktop::from_env("CURRENT_DESKTOP");
        if !desktop.is_unknown() {
            return desktop;
        }

        let desktop = Desktop::from_env("XDG_SESSION_DESKTOP");
        if !desktop.is_unknown() {
            return desktop;
        }

        let desktop = Desktop::from_env("XDG_CURRENT_DESKTOP");
        if !desktop.is_unknown() {
            return desktop;
        }

        Desktop::Unknown
    }

    fn is_unknown(&self) -> bool {
        matches!(*self, Desktop::Unknown)
    }
}
