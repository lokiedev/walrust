use std::{fs, io, path::Path};

pub fn get_files_in_dir(dir_path: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    let path = Path::new(dir_path);

    if !path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Path: {}, is not a valid directory!", dir_path),
        ));
    }

    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;

        if entry.metadata()?.is_file() {
            files.push(entry.path().display().to_string());
        }
    }

    Ok(files)
}
