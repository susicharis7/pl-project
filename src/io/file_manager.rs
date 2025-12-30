use std::fs;
use std::io::{Read, Write};
use std::path::Path;

pub fn ensure_saves_dir() -> std::io::Result<()> {
    fs::create_dir_all("saves")?;
    Ok(())
}

pub fn write_to_file(path: &str, contents: &str) -> std::io::Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = fs::File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn read_from_file(path: &str) -> std::io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
