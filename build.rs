use std::env;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() {
    let out_dir = String::from(env::var_os("OUT_DIR").unwrap().to_string_lossy());
    let mut display_root = PathBuf::new();
    display_root.push(env!("CARGO_MANIFEST_DIR"));
    display_root.push("res/");
    let cargo_mainfest_dir = String::from(display_root.to_string_lossy());
    for entry in WalkDir::new(&cargo_mainfest_dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        let f_path = entry.path();
        let f_name = String::from(f_path.to_string_lossy());
        let mut out_name = PathBuf::new();
        out_name.push(&out_dir);
        out_name.push("res/");
        out_name.push(&f_name.strip_prefix(&cargo_mainfest_dir).unwrap());
        if !out_name.exists() {
            if f_path.is_file() {
                fs::copy(f_name, out_name).unwrap();
            } else {
                fs::create_dir(out_name).unwrap();
            }   
        }
    }
}
