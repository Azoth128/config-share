use std::fs;

#[tokio::main]
async fn main() -> () {
    let files_to_encrypt = get_files();

    print!("{:?}", files_to_encrypt);
}

fn get_files() -> Vec<String> {
    fs::read_dir(".")
        .unwrap()
        .map(|entry| {
            entry.unwrap().file_name().to_str().map(|e| String::from(e))
        })
        .filter_map(|e| e)
        .collect()
}
