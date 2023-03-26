use std::fs::{self, DirEntry};

#[tokio::main]
async fn main() -> () {
    let files_to_encrypt = get_files(".");

    for file in files_to_encrypt {
        println!("{}", file);
    }
}

fn with_files_in_dirs(entry: DirEntry) -> Vec<String> {
    let file_type = entry.file_type().unwrap();
    let file_name = match entry.path().to_str().map(|e| String::from(e)) {
        Some(str) => str,
        None => return vec![],
    };

    if file_type.is_file() {
        vec![file_name]
    } else if file_type.is_dir() {
        get_files(&file_name)
    } else {
        vec![]
    }
}

fn get_files(path: &str) -> Vec<String> {
    fs::read_dir(path)
        .unwrap()
        .map(|entry| entry.unwrap())
        .flat_map(|entry| with_files_in_dirs(entry))
        .collect()
}
