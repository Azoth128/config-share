use sha::sha256::ops::digest;
#[derive(Debug)]
pub struct File {
    file_name: String,
    hash: Option<[u32; 8]>,
}

impl File {
    pub fn new(file_name: &str) -> File {
        let read_result = std::fs::read(file_name);
        let hash = match read_result {
            Ok(bytes) => Some(digest(&bytes)),
            Err(_) => None,
        };

        File {
            file_name: file_name.to_string(),
            hash,
        }
    }
}
