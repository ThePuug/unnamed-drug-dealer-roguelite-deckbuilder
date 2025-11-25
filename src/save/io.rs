// File I/O operations for save system.

use super::crypto;
use super::types::*;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Get platform-appropriate save directory
pub fn get_save_directory() -> PathBuf {
    let base = dirs::data_local_dir()
        .or_else(dirs::data_dir)
        .unwrap_or_else(|| PathBuf::from("."));

    let save_dir = base.join("DrugDealerDeckbuilder");

    // Ensure directory exists
    if !save_dir.exists() {
        let _ = fs::create_dir_all(&save_dir);
    }

    save_dir
}

/// Save data atomically with backup
pub fn save_atomic(
    save_path: &Path,
    backup_path: &Path,
    data: &SaveData,
) -> Result<(), SaveError> {
    // Serialize payload
    let payload = bincode::serialize(data)
        .map_err(|e| SaveError::SerializationError(e.to_string()))?;

    // Sign payload
    let signature = crypto::sign(&payload);

    // Create save file structure
    let save_file = SaveFile {
        version: SAVE_VERSION,
        data: payload,
        signature,
    };

    // Serialize entire save file
    let file_bytes = bincode::serialize(&save_file)
        .map_err(|e| SaveError::SerializationError(e.to_string()))?;

    // Write to temp file first
    let temp_path = save_path.with_extension("tmp");

    let mut temp_file = fs::File::create(&temp_path)
        .map_err(|e| SaveError::IoError(e.to_string()))?;

    temp_file
        .write_all(&file_bytes)
        .map_err(|e| SaveError::IoError(e.to_string()))?;

    temp_file
        .sync_all()
        .map_err(|e| SaveError::IoError(e.to_string()))?;

    drop(temp_file);

    // Backup existing save (ignore errors if no existing save)
    if save_path.exists() {
        let _ = fs::copy(save_path, backup_path);
    }

    // Atomic rename: temp -> actual
    fs::rename(&temp_path, save_path)
        .map_err(|e| SaveError::IoError(e.to_string()))?;

    Ok(())
}

/// Load save file with signature verification
pub fn load_save(path: &Path) -> Result<SaveData, SaveError> {
    // Check file exists
    if !path.exists() {
        return Err(SaveError::NotFound);
    }

    // Read file
    let file_bytes = fs::read(path)
        .map_err(|e| SaveError::IoError(e.to_string()))?;

    // Deserialize save file structure
    let save_file: SaveFile = bincode::deserialize(&file_bytes)
        .map_err(|_| SaveError::TamperedOrCorrupted)?;

    // Check version
    if save_file.version > SAVE_VERSION {
        return Err(SaveError::UnsupportedVersion(save_file.version));
    }

    // Verify signature
    if !crypto::verify(&save_file.data, &save_file.signature) {
        return Err(SaveError::TamperedOrCorrupted);
    }

    // Deserialize payload
    let data: SaveData = bincode::deserialize(&save_file.data)
        .map_err(|_| SaveError::TamperedOrCorrupted)?;

    // Validate data sanity
    data.validate()?;

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_save_and_load_roundtrip() {
        let dir = tempdir().unwrap();
        let save_path = dir.path().join("save.dat");
        let backup_path = dir.path().join("save.dat.bak");

        let mut data = SaveData::new();
        data.character = Some(CharacterState::new());

        save_atomic(&save_path, &backup_path, &data).unwrap();

        let loaded = load_save(&save_path).unwrap();
        assert!(loaded.character.is_some());
    }

    #[test]
    fn test_backup_created_on_overwrite() {
        let dir = tempdir().unwrap();
        let save_path = dir.path().join("save.dat");
        let backup_path = dir.path().join("save.dat.bak");

        // First save
        let mut data1 = SaveData::new();
        data1.character = Some(CharacterState::new());
        save_atomic(&save_path, &backup_path, &data1).unwrap();

        // Modify and save again
        let mut data2 = data1.clone();
        if let Some(ref mut c) = data2.character {
            c.heat = 50;
        }
        save_atomic(&save_path, &backup_path, &data2).unwrap();

        // Backup should exist
        assert!(backup_path.exists());

        // Backup should have original data (heat = 0)
        let backup_data = load_save(&backup_path).unwrap();
        assert_eq!(backup_data.character.unwrap().heat, 0);

        // Current save should have new data (heat = 50)
        let current_data = load_save(&save_path).unwrap();
        assert_eq!(current_data.character.unwrap().heat, 50);
    }

    #[test]
    fn test_load_nonexistent_file() {
        let dir = tempdir().unwrap();
        let save_path = dir.path().join("nonexistent.dat");

        let result = load_save(&save_path);
        assert!(matches!(result, Err(SaveError::NotFound)));
    }

    #[test]
    fn test_tampered_file_rejected() {
        let dir = tempdir().unwrap();
        let save_path = dir.path().join("save.dat");
        let backup_path = dir.path().join("save.dat.bak");

        let data = SaveData::new();
        save_atomic(&save_path, &backup_path, &data).unwrap();

        // Tamper with file - modify any byte that exists
        let mut bytes = fs::read(&save_path).unwrap();
        assert!(!bytes.is_empty(), "Save file should not be empty");
        // Tamper near the end to definitely hit data/signature
        let idx = bytes.len().saturating_sub(5).max(0);
        bytes[idx] = bytes[idx].wrapping_add(1);
        fs::write(&save_path, &bytes).unwrap();

        let result = load_save(&save_path);
        assert!(matches!(result, Err(SaveError::TamperedOrCorrupted)));
    }

    #[test]
    fn test_get_save_directory_creates_dir() {
        let dir = get_save_directory();
        assert!(dir.exists());
    }
}
