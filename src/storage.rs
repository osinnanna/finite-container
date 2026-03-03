use std::fs;
use std::path::Path;
use std::io::Result;

pub fn save_file(container_id: &str, filename: &str, bytes: &[u8]) -> Result<()> {
    let dir_path = format!("./containers/{}", container_id);
    fs::create_dir_all(&dir_path)?;
    let file_path = format!("{}/{}", dir_path, filename);
    fs::write(file_path, bytes);
    Ok(())
} 

pub fn list_files(container_id: &str) -> Vec<String> {
    let dir_path = format!("./containers/{}", container_id);
    if let Ok(entries) = fs::read_dir(dir_path) {
        entries.filter_map(|e| e.ok())
            .filter_map(|e| e.file_name().into_string().ok())
            .collect()
    } else {
        vec![]
    }
}
