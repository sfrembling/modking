use sha2::Digest;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

/// Hashes a file using SHA-256
pub fn hash(file: &Path) -> anyhow::Result<String> {
    if !file.exists() {
        return Err(anyhow::anyhow!("File does not exist"));
    }

    let file = File::open(file)?;
    let mut reader = BufReader::new(file);
    let mut hasher = sha2::Sha256::new();

    let mut buffer = [0; 1024];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash_result = hasher.finalize();

    Ok(format!("{:x}", hash_result))
}
