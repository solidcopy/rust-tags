use std::path::Path;

use once_cell::sync::Lazy;

/// tagsファイル名
pub static TAGS_FILENAME: Lazy<&Path> = Lazy::new(|| Path::new("tags"));

/// 対象フォルダパス
pub static TARGET_FOLDER: Lazy<&Path> = Lazy::new(|| Path::new("."));
