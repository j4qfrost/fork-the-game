use std::path::PathBuf;

pub fn from_out_dir(file_name: &str) -> String {
    let mut display_root = PathBuf::new();
    display_root.push(env!("OUT_DIR"));
    display_root.push(file_name);
    display_root.to_str().unwrap().to_string()
}
