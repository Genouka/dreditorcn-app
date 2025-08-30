use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{command};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveError {
    UnsupportedPlatform,
    UnknownChapter,
    UnknownSubChapter,
    FileNotFound,
    IoError(String),
}

impl From<std::io::Error> for SaveError {
    fn from(err: std::io::Error) -> Self {
        SaveError::IoError(err.to_string())
    }
}


/// 根据平台返回存档目录
#[command]
pub fn get_save_directory() -> Result<String, SaveError> {
    let dir = if cfg!(target_os = "windows") {
        dirs::data_local_dir()
            .map(|d| d.join("DELTARUNE"))
            .ok_or(SaveError::UnsupportedPlatform)?
    } else if cfg!(target_os = "macos") {
        dirs::home_dir()
            .map(|h| {
                h.join("Library")
                    .join("Application Support")
                    .join("com.tobyfox.deltarune")
            })
            .ok_or(SaveError::UnsupportedPlatform)?
    } else if cfg!(target_os = "linux") {
        dirs::home_dir()
            .map(|d| {
                d.join(".config")
                .join("DELTARUNE")
            })
            .ok_or(SaveError::UnsupportedPlatform)?
    }
    else{
        return Err(SaveError::UnsupportedPlatform);
    };
    Ok(dir.to_string_lossy().into_owned())
}

/// 根据 href 计算文件名后缀
fn file_suffix_from_href(href: &str) -> Result<String, SaveError> {
    let chapter = if href.contains("ch1") {
        "1"
    } else if href.contains("ch2") {
        "2"
    } else if href.contains("ch3") {
        "3"
    } else if href.contains("ch4") {
        "4"
    } else {
        return Err(SaveError::UnknownChapter);
    };

    let sub = if href.contains("ss1") {
        "0"
    } else if href.contains("ss2") {
        "1"
    } else if href.contains("ss3") {
        "2"
    } else {
        return Err(SaveError::UnknownSubChapter);
    };

    Ok(format!("filech{}_{}", chapter, sub))
}

/// 拼接完整路径
fn full_path(href: &str) -> Result<String, SaveError> {
    let save_dir = get_save_directory()?;
    let suffix = file_suffix_from_href(href)?;
    Ok(PathBuf::from(save_dir).join(suffix).to_string_lossy().into_owned())
}

#[command]
pub async fn read_file_11(href: String) -> Result<String, SaveError> {
    let path = full_path(&href)?;
    let content = fs::read_to_string(&path).map_err(|_| SaveError::FileNotFound)?;
    Ok(content)
}

#[command]
pub async fn write_sync_t(href: String, text: String) -> Result<bool, SaveError> {
    let path = full_path(&href)?;
    if let Some(parent) = PathBuf::from(&path).parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, text)?;
    Ok(true)
}

#[command]
pub fn get_platform() -> String {
    if cfg!(target_os = "windows") {
        "win32".into()
    } else if cfg!(target_os = "macos") {
        "darwin".into()
    } else if cfg!(target_os = "linux") {
        "linux".into()
    } else {
        "unknown".into()
    }
}