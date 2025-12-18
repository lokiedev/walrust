use std::path::PathBuf;

pub fn is_image_file(file_name: &PathBuf) -> bool {
    let extension = match file_name.extension() {
        Some(ext) => ext,
        None => return false,
    };

    let ext_lowercase = extension.to_ascii_lowercase();

    matches!(
        ext_lowercase.to_str(),
        Some("jpg") | Some("jpeg") | Some("png") | Some("webp")
    )
}
