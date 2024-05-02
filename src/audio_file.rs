use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;

use crate::model::AlbumInfo;
use crate::tags;
use crate::tags::{TagIO, Tags};
use anyhow::Result;
use thiserror::Error;

/// 音楽ファイル
pub struct AudioFile {
    filepath: PathBuf,
    tag_io: Box<dyn TagIO>,
}

/// 指定されたフォルダの音楽ファイルを一覧にする。
pub fn find_audio_files(folder: &Path) -> Result<Vec<AudioFile>> {
    let mut audio_files = vec![];

    for filepath in find_files(folder)? {
        if let Some(tag_io) = tags::tag_io_for(filepath.as_path()) {
            let audio_file = AudioFile::new(filepath, tag_io);
            audio_files.push(audio_file);
        }
    }

    Ok(audio_files)
}

/// 指定されたフォルダのファイルを一覧にする。
/// ファイル名の昇順でソートする。
fn find_files(folder: &Path) -> Result<Vec<PathBuf>> {
    let mut filepaths = vec![];
    match folder.read_dir() {
        Ok(entries) => {
            for entry in entries {
                let filepath = entry.unwrap().path();
                if filepath.is_file() {
                    filepaths.push(filepath);
                }
            }
        }
        Err(_) => Err(FileAccessError::INSTANCE(
            "ファイル一覧が取得できませんでした",
        ))?,
    }

    filepaths.sort();

    Ok(filepaths)
}

/// アルバム情報で音楽ファイルのタグ情報を更新する。
pub fn update_by_album_info(audio_files: &mut Vec<AudioFile>, album: &AlbumInfo) -> Result<()> {
    let mut audio_file_iter = audio_files.iter();

    for (disc_index, disc) in album.discs().iter().enumerate() {
        let disc_number = disc_index + 1;
        for (track_index, track) in disc.tracks().iter().enumerate() {
            let track_number = track_index + 1;

            let audio_file = match audio_file_iter.next() {
                Some(audio_file) => audio_file,
                None => Err(TitlesMismatchFilesError::INSTANCE)?,
            };

            let mut tags = Tags::new();
            tags.update_album_info(&album);
            tags.update_disc_info(disc_number, &disc);
            tags.update_track_info(track_number, &track);

            audio_file.save_tags(&tags)?;
        }
    }

    if let Some(_) = audio_file_iter.next() {
        Err(TitlesMismatchFilesError::INSTANCE)?;
    }

    Ok(())
}

/// 音楽ファイルのタグ情報を元にアルバム情報を作成する。
pub fn to_album_info(audio_files: &Vec<AudioFile>) -> Result<AlbumInfo> {
    let first_file = match audio_files.first() {
        Some(audio_file) => audio_file,
        None => Err(NoAudioFileError::INSTANCE)?,
    };

    let tags = first_file.load_tags()?;
    let album = tags.album().map(String::from);
    let album_artist = tags.album_artist().map(String::from);
    let release_date = tags.release_date().map(String::from);
    let mut album_info = AlbumInfo::new(album, album_artist, release_date);

    let art_work = match tags.art_work() {
        Some(art_work) => Some(art_work.clone()),
        None => None,
    };
    album_info.set_art_work(art_work);

    // ディスク番号が設定されていても無視して1つのディスク情報に全トラック情報を格納する
    let disc_info = album_info.new_disc();

    for audio_file in audio_files {
        let tags = audio_file.load_tags()?;

        let title = match tags.title() {
            Some(title) => title,
            None => audio_file.filepath.file_stem().unwrap().to_str().unwrap(),
        };
        disc_info.new_track(Some(title.to_string()), tags.artists().clone());
    }

    Ok(album_info)
}

impl AudioFile {
    /// 音楽ファイルを作成する。
    pub fn new(filepath: PathBuf, tag_io: Box<dyn TagIO>) -> AudioFile {
        AudioFile { filepath, tag_io }
    }

    /// ファイルからタグ情報を取得する。
    pub fn load_tags(&self) -> Result<Tags> {
        self.tag_io.load(self.filepath.as_path())
    }

    /// ファイルにタグ情報を保存する。
    pub fn save_tags(&self, tags: &Tags) -> Result<()> {
        self.tag_io.save(self.filepath.as_path(), tags)
    }

    /// タグ情報を元にファイルをリネームする。
    pub fn rename(&mut self) -> Result<()> {
        let tags = self.load_tags()?;

        let mut filename = String::new();

        if tags.number_of_discs().unwrap_or(1) > 1 {
            let disc_number =
                add_zero_paddings(tags.disc_number().unwrap(), tags.number_of_discs().unwrap());
            write!(filename, "{}.", disc_number)?;
        };

        let track_number = add_zero_paddings(
            tags.track_number().unwrap(),
            tags.number_of_tracks().unwrap(),
        );
        write!(filename, "{}.", track_number.as_str())?;

        filename.push_str(tags.title().unwrap());

        if let Some(extension) = self.filepath.extension() {
            let extension = extension.to_str().unwrap();
            write!(filename, ".{}", extension)?;
        }

        let filename = Regex::new("[*\\\\|:\"<>/?]")
            .unwrap()
            .replace_all(filename.as_str(), "");

        let mut new_filepath = self.filepath.clone();
        new_filepath.set_file_name(&*filename);

        fs::rename(self.filepath.as_path(), new_filepath.as_path())?;

        self.filepath = new_filepath;

        Ok(())
    }
}

fn add_zero_paddings(n: usize, max: usize) -> String {
    let mut n = n.to_string();
    let max = max.to_string();
    while n.len() < max.len() {
        n.insert(0, '0');
    }
    n
}

/// タイトル数ファイル数不一致エラー
///
/// tagsファイルに書かれているタイトル数と対象となる音楽ファイル数が一致しない場合に発生する。
#[derive(Debug, Error)]
pub enum TitlesMismatchFilesError {
    #[error("対象ファイル数とtagsのタイトル数が一致しません")]
    INSTANCE,
}

/// ファイルアクセスエラー
#[derive(Debug, Error)]
pub enum FileAccessError {
    #[error("{0}")]
    INSTANCE(&'static str),
}

/// 音楽ファイル不在エラー
///
/// 音楽ファイルがない場合に発生する。
#[derive(Debug, Error)]
pub enum NoAudioFileError {
    #[error("対象ファイルがありません")]
    INSTANCE,
}
