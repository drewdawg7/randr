use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::player::Player;
use crate::storage::Storage;
use super::state::GameSaveState;

/// Errors that can occur during save/load operations
#[derive(Debug)]
pub enum SaveLoadError {
    IoError(io::Error),
    SerializationError(serde_json::Error),
    SaveFileNotFound,
    CorruptedSaveFile(String),
    InvalidSaveVersion(u32),
}

impl From<io::Error> for SaveLoadError {
    fn from(err: io::Error) -> Self {
        SaveLoadError::IoError(err)
    }
}

impl From<serde_json::Error> for SaveLoadError {
    fn from(err: serde_json::Error) -> Self {
        SaveLoadError::SerializationError(err)
    }
}

impl std::fmt::Display for SaveLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveLoadError::IoError(e) => write!(f, "IO Error: {}", e),
            SaveLoadError::SerializationError(e) => write!(f, "Serialization Error: {}", e),
            SaveLoadError::SaveFileNotFound => write!(f, "Save file not found"),
            SaveLoadError::CorruptedSaveFile(msg) => write!(f, "Corrupted save file: {}", msg),
            SaveLoadError::InvalidSaveVersion(v) => write!(f, "Invalid save version: {}", v),
        }
    }
}

impl std::error::Error for SaveLoadError {}

/// Get the default save directory path
pub fn get_save_directory() -> Result<PathBuf, SaveLoadError> {
    // Use platform-appropriate directory
    let base_dir = if cfg!(target_os = "macos") {
        dirs::home_dir()
            .map(|p| p.join("Library/Application Support"))
            .ok_or_else(|| SaveLoadError::IoError(io::Error::new(
                io::ErrorKind::NotFound,
                "Could not determine home directory"
            )))?
    } else if cfg!(target_os = "windows") {
        dirs::data_local_dir()
            .ok_or_else(|| SaveLoadError::IoError(io::Error::new(
                io::ErrorKind::NotFound,
                "Could not determine app data directory"
            )))?
    } else {
        // Linux/Unix
        dirs::data_dir()
            .ok_or_else(|| SaveLoadError::IoError(io::Error::new(
                io::ErrorKind::NotFound,
                "Could not determine data directory"
            )))?
    };

    let save_dir = base_dir.join("R&R");

    // Create directory if it doesn't exist
    if !save_dir.exists() {
        fs::create_dir_all(&save_dir)?;
    }

    Ok(save_dir)
}

/// Get the default save file path
pub fn get_save_file_path() -> Result<PathBuf, SaveLoadError> {
    Ok(get_save_directory()?.join("save.json"))
}

/// Save the game state to a file
pub fn save_game(
    player: &Player,
    storage: &Storage,
    save_path: Option<&Path>,
) -> Result<(), SaveLoadError> {
    let save_path = match save_path {
        Some(p) => p.to_path_buf(),
        None => get_save_file_path()?,
    };

    // Create the save state
    let save_state = GameSaveState::from_game(player, storage);

    // Serialize to JSON with pretty printing
    let json = serde_json::to_string_pretty(&save_state)?;

    // Write to file
    fs::write(&save_path, json)?;

    println!("Game saved to: {}", save_path.display());
    Ok(())
}

/// Load the game state from a file
pub fn load_game(
    save_path: Option<&Path>,
) -> Result<(Player, Storage), SaveLoadError> {
    let save_path = match save_path {
        Some(p) => p.to_path_buf(),
        None => get_save_file_path()?,
    };

    // Check if save file exists
    if !save_path.exists() {
        return Err(SaveLoadError::SaveFileNotFound);
    }

    // Read the file
    let json = fs::read_to_string(&save_path)?;

    // Deserialize
    let save_state: GameSaveState = serde_json::from_str(&json)
        .map_err(|e| SaveLoadError::CorruptedSaveFile(e.to_string()))?;

    // Verify version
    if save_state.version != GameSaveState::CURRENT_VERSION {
        return Err(SaveLoadError::InvalidSaveVersion(save_state.version));
    }

    // Convert back to game structures
    let player = save_state.to_player();
    let storage = save_state.to_storage();

    println!("Game loaded from: {}", save_path.display());
    Ok((player, storage))
}

/// Check if a save file exists
pub fn save_exists(save_path: Option<&Path>) -> bool {
    let path = match save_path {
        Some(p) => p.to_path_buf(),
        None => match get_save_file_path() {
            Ok(p) => p,
            Err(_) => return false,
        },
    };
    path.exists()
}

/// Delete the save file
pub fn delete_save(save_path: Option<&Path>) -> Result<(), SaveLoadError> {
    let save_path = match save_path {
        Some(p) => p.to_path_buf(),
        None => get_save_file_path()?,
    };

    if save_path.exists() {
        fs::remove_file(&save_path)?;
        println!("Save file deleted: {}", save_path.display());
    }

    Ok(())
}

// Helper to use dirs crate - we need to add it to Cargo.toml
// For now, provide a basic fallback
mod dirs {
    use std::path::PathBuf;

    pub fn home_dir() -> Option<PathBuf> {
        std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
    }

    pub fn data_dir() -> Option<PathBuf> {
        std::env::var("HOME")
            .ok()
            .map(|h| PathBuf::from(h).join(".local/share"))
    }

    pub fn data_local_dir() -> Option<PathBuf> {
        std::env::var("LOCALAPPDATA")
            .ok()
            .map(PathBuf::from)
    }
}
