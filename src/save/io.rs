// File I/O operations for save system.

use super::crypto;
use super::types::*;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Get platform-appropriate save directory.
/// SOW-023: `DDD_SAVE_DIR` overrides it so e2e playtests and dev tooling
/// never touch the real save (a scripted bust once permadeathed a live
/// character).
pub fn get_save_directory() -> PathBuf {
    let save_dir = match std::env::var("DDD_SAVE_DIR") {
        Ok(dir) if !dir.trim().is_empty() => PathBuf::from(dir),
        _ => dirs::data_local_dir()
            .or_else(dirs::data_dir)
            .unwrap_or_else(|| PathBuf::from("."))
            .join("DrugDealerDeckbuilder"),
    };

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
    // SOW-021: Reject any mismatched version (older or newer). There is no
    // migration path pre-release; load_or_create falls back to a fresh account.
    if save_file.version != SAVE_VERSION {
        return Err(SaveError::UnsupportedVersion(save_file.version));
    }

    // Verify signature
    if !crypto::verify(&save_file.data, &save_file.signature) {
        return Err(SaveError::TamperedOrCorrupted);
    }

    // Deserialize payload
    let mut data: SaveData = bincode::deserialize(&save_file.data)
        .map_err(|_| SaveError::TamperedOrCorrupted)?;

    // SOW-031: normalize content-decision drift (kingpin silhouette)
    data.normalize();

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

        let data = SaveData::new();

        save_atomic(&save_path, &backup_path, &data).unwrap();

        let loaded = load_save(&save_path).unwrap();
        assert_eq!(loaded.dealers.len(), 1);
        assert!(loaded.dealers[0].is_kingpin);
    }

    #[test]
    fn test_backup_created_on_overwrite() {
        let dir = tempdir().unwrap();
        let save_path = dir.path().join("save.dat");
        let backup_path = dir.path().join("save.dat.bak");

        // First save
        let data1 = SaveData::new();
        save_atomic(&save_path, &backup_path, &data1).unwrap();

        // Modify and save again
        let mut data2 = data1.clone();
        data2.active_character_mut().heat = 50;
        save_atomic(&save_path, &backup_path, &data2).unwrap();

        // Backup should exist
        assert!(backup_path.exists());

        // Backup should have original data (heat = 0)
        let backup_data = load_save(&backup_path).unwrap();
        assert_eq!(backup_data.active_character().heat, 0);

        // Current save should have new data (heat = 50)
        let current_data = load_save(&save_path).unwrap();
        assert_eq!(current_data.active_character().heat, 50);
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

    #[test]
    fn test_old_version_rejected() {
        // SOW-021: version mismatch (older or newer) must be rejected so
        // load_or_create falls back to a fresh account instead of
        // misinterpreting stale data (e.g., pre-spec upgrade thresholds).
        let dir = tempdir().unwrap();
        let save_path = dir.path().join("save.dat");

        let payload = bincode::serialize(&SaveData::new()).unwrap();
        let signature = crypto::sign(&payload);
        let old_file = SaveFile {
            version: SAVE_VERSION - 1,
            data: payload,
            signature,
        };
        fs::write(&save_path, bincode::serialize(&old_file).unwrap()).unwrap();

        let result = load_save(&save_path);
        assert!(matches!(result, Err(SaveError::UnsupportedVersion(v)) if v == SAVE_VERSION - 1));
    }
}
