use std::path::{Path, PathBuf};

use once_cell::sync::Lazy;

/// 共通の戻り値型
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// tagsファイル名
pub static TAGS_FILEPATH: Lazy<PathBuf> = Lazy::new(|| Path::new("tags").canonicalize().unwrap());

/// 対象フォルダパス
pub static TARGET_FOLDER: Lazy<&Path> = Lazy::new(|| Path::new("."));
