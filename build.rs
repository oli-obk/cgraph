use std::fs::read_dir;

fn main() {
    let dirs = read_dir("C:\\Program Files (x86)\\")
        .and_then(|dirs| read_dir("C:\\Program Files\\").map(|dirs2| dirs.chain(dirs2)));
    if let Ok(dirs) = dirs {
        for entry in dirs {
            if let Ok(entry) = entry {
                if let Some(file) = entry.file_name().to_str() {
                    if file.starts_with("Graphviz") {
                        let path = entry.path();
                        if let Some(path) = path.to_str() {
                            println!("cargo:rustc-link-search={}\\lib", path);
                            println!("cargo:rustc-link-search={}\\lib\\debug\\lib", path);
                        }
                    }
                }
            }
        }
    }
}
