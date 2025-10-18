use std::path::PathBuf;

#[allow(dead_code)]
pub fn scan_directory(path: &PathBuf) -> Vec<PathBuf> {
    let mut results = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                if meta.is_file() {
                    results.push(entry.path());
                }
            }
        }
    }
    
    results
}

#[allow(dead_code)]
pub fn ensure_directory(path: &PathBuf) -> std::io::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}
