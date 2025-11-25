// Save system for character state and game progress persistence.

mod types;
mod crypto;
mod io;

pub use types::*;

use bevy::prelude::*;
use std::path::PathBuf;

/// Plugin that manages save/load operations
pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        // Initialize save manager resource
        app.insert_resource(SaveManager::new());
    }
}

/// Resource that manages save operations
#[derive(Resource)]
pub struct SaveManager {
    save_path: PathBuf,
    backup_path: PathBuf,
}

impl SaveManager {
    pub fn new() -> Self {
        let save_dir = io::get_save_directory();
        Self {
            save_path: save_dir.join("save.dat"),
            backup_path: save_dir.join("save.dat.bak"),
        }
    }

    /// Save current game state
    pub fn save(&self, data: &SaveData) -> Result<(), SaveError> {
        io::save_atomic(&self.save_path, &self.backup_path, data)
    }

    /// Load game state, attempting backup recovery if primary fails
    pub fn load(&self) -> Result<SaveData, SaveError> {
        match io::load_save(&self.save_path) {
            Ok(data) => Ok(data),
            Err(SaveError::NotFound) => Err(SaveError::NotFound),
            Err(e) => {
                // Try backup recovery
                warn!("Primary save failed ({:?}), attempting backup recovery", e);
                io::load_save(&self.backup_path)
            }
        }
    }

    /// Load game state or create new if none exists
    pub fn load_or_create(&self) -> SaveData {
        match self.load() {
            Ok(data) => data,
            Err(SaveError::NotFound) => {
                info!("No save file found, creating new game state");
                SaveData::new()
            }
            Err(e) => {
                warn!("Save corrupted or tampered ({:?}), starting fresh", e);
                SaveData::new()
            }
        }
    }

    /// Check if a save file exists
    pub fn save_exists(&self) -> bool {
        self.save_path.exists()
    }

    /// Delete save file (for permadeath)
    pub fn delete_character(&self, data: &mut SaveData) -> Result<(), SaveError> {
        data.character = None;
        self.save(data)
    }
}

impl Default for SaveManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn test_save_manager() -> (SaveManager, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let manager = SaveManager {
            save_path: dir.path().join("save.dat"),
            backup_path: dir.path().join("save.dat.bak"),
        };
        (manager, dir)
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let (manager, _dir) = test_save_manager();

        let mut data = SaveData::new();
        data.character = Some(CharacterState::new());

        manager.save(&data).unwrap();
        let loaded = manager.load().unwrap();

        assert!(loaded.character.is_some());
        assert_eq!(loaded.character.unwrap().heat, 0);
    }

    #[test]
    fn test_load_nonexistent_returns_not_found() {
        let (manager, _dir) = test_save_manager();

        let result = manager.load();
        assert!(matches!(result, Err(SaveError::NotFound)));
    }

    #[test]
    fn test_load_or_create_returns_new_when_no_save() {
        let (manager, _dir) = test_save_manager();

        let data = manager.load_or_create();
        assert!(data.character.is_none());
    }

    #[test]
    fn test_backup_recovery() {
        let (manager, _dir) = test_save_manager();

        // Create and save valid data
        let mut data = SaveData::new();
        data.character = Some(CharacterState::new());
        manager.save(&data).unwrap();

        // Save again to create backup (backup only created on overwrite)
        manager.save(&data).unwrap();

        // Corrupt primary save
        fs::write(&manager.save_path, b"corrupted data").unwrap();

        // Load should recover from backup
        let loaded = manager.load().unwrap();
        assert!(loaded.character.is_some());
    }

    #[test]
    fn test_tampered_save_rejected() {
        let (manager, _dir) = test_save_manager();

        // Create and save valid data twice so backup exists
        let data = SaveData::new();
        manager.save(&data).unwrap();
        manager.save(&data).unwrap();

        // Tamper with both primary and backup
        let mut bytes = fs::read(&manager.save_path).unwrap();
        assert!(!bytes.is_empty(), "Save file should not be empty");
        let idx = bytes.len().saturating_sub(5).max(0);
        bytes[idx] = bytes[idx].wrapping_add(1);
        fs::write(&manager.save_path, &bytes).unwrap();

        // Also tamper backup
        let mut backup_bytes = fs::read(&manager.backup_path).unwrap();
        let backup_idx = backup_bytes.len().saturating_sub(5).max(0);
        backup_bytes[backup_idx] = backup_bytes[backup_idx].wrapping_add(1);
        fs::write(&manager.backup_path, &backup_bytes).unwrap();

        // Load should fail with tampering error (both files tampered)
        let result = manager.load();
        assert!(matches!(result, Err(SaveError::TamperedOrCorrupted)));
    }

    #[test]
    fn test_delete_character() {
        let (manager, _dir) = test_save_manager();

        // Create save with character
        let mut data = SaveData::new();
        data.character = Some(CharacterState::new());
        manager.save(&data).unwrap();

        // Delete character
        manager.delete_character(&mut data).unwrap();

        // Verify character is gone
        let loaded = manager.load().unwrap();
        assert!(loaded.character.is_none());
    }
}
