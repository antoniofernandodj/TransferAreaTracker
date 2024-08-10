use std::fs;
use std::io::{self, Read};
use std::path::Path;
use walkdir::WalkDir;

const SRC_DIR: &str = "./teste";


fn get_all_bytes<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut all_bytes = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            all_bytes.extend(get_all_bytes(&path)?);
        } else {
            let mut file = fs::File::open(&path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            all_bytes.extend(buffer);
        }
    }

    Ok(all_bytes)
}

fn count_files_in_dir(dir: &str) -> usize {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .count()
}

fn get_dir_state(dir: &str) -> Result<(Vec<u8>, usize), Box<dyn std::error::Error>>{
    let state = (get_all_bytes(dir)?, count_files_in_dir(dir));
    return Ok(state)
}

fn changed() {
    println!("Directory Changed!");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (mut last_bytes, mut last_files) = get_dir_state(SRC_DIR)?;

    println!("Running...");
    loop {
        
        let (bytes, files) = get_dir_state(SRC_DIR)?;

        if !(bytes == last_bytes) || !(files == last_files) {
            changed();

        }

        (last_bytes, last_files) = (bytes, files);
    }
}
