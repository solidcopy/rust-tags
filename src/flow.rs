use std::fmt;
use std::path::Path;

use crate::audio_file::AudioFile;
use crate::common::{Result, TAGS_FILENAME, TARGET_FOLDER};
use crate::{audio_file, tags_file};

/// tagsファイルの有無で実行する処理を分岐する。
pub fn execute() -> Result<()> {
    // tagsファイルの有無でインポート/エクスポートのどちらかを実行する
    if TAGS_FILENAME.exists() {
        import_flow()?;
        rename_flow()?;
    } else {
        export_flow()?;
    }

    Ok(())
}

/// インポート処理を実行する。
fn import_flow() -> Result<()> {
    println!("インポート処理を開始します。");

    let album_info = tags_file::load_tags_file(&TAGS_FILENAME)?;

    let mut audio_files = require_audio_files(&TARGET_FOLDER)?;

    audio_file::update_by_album_info(&mut audio_files, &album_info)?;

    println!("インポート処理を完了しました。");

    Ok(())
}

/// リネーム処理を実行する。
fn rename_flow() -> Result<()> {
    println!("リネーム処理を開始します。");

    let mut audio_files = require_audio_files(&TARGET_FOLDER)?;

    for audio_file in audio_files.iter_mut() {
        audio_file.rename()?;
    }

    println!("リネーム処理を完了しました。");

    Ok(())
}

fn require_audio_files(folder: &Path) -> Result<Vec<AudioFile>> {
    let audio_files = audio_file::find_audio_files(folder)?;

    if audio_files.len() == 0 {
        return NoTargetError.into();
    }

    Ok(audio_files)
}

/// エクスポート処理を実行する。
fn export_flow() -> Result<()> {
    println!("エクスポート処理を開始します。");

    let audio_files = audio_file::find_audio_files(&TARGET_FOLDER)?;

    let album_info = audio_file::to_album_info(&audio_files)?;

    tags_file::write_tags_file(&TAGS_FILENAME, &album_info)?;

    tags_file::write_art_work_file(album_info.art_work())?;

    println!("エクスポート処理を完了しました。");

    Ok(())
}

/// 音楽ファイル不在エラー
///
/// 処理する音楽ファイルがない場合に発生する。
#[derive(Debug, Clone)]
pub struct NoTargetError;

impl fmt::Display for NoTargetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "処理対象がありません")
    }
}

impl std::error::Error for NoTargetError {}

impl From<NoTargetError> for Result<Vec<AudioFile>> {
    fn from(error: NoTargetError) -> Self {
        Err(Box::new(error))
    }
}
