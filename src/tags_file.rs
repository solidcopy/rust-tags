use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use regex;
use regex::Regex;
use unicode_normalization::UnicodeNormalization;

use crate::common::Result;
use crate::model::{AlbumInfo, DiscInfo};
use crate::tags::{Image, ImageFormat};

/// tagsファイルを読み込んでアルバム情報を作成する。
pub fn load_tags_file(tags_filepath: &Path) -> Result<AlbumInfo> {
    let tags_file_contents = read_tags_file(tags_filepath)?;

    let mut lines = tags_file_contents.lines();
    let lines = &mut lines;

    let album = read_line(lines.next(), "アルバム名がありません。")?;
    let album_artist = read_line(lines.next(), "アルバムアーティスト名がありません。")?;
    let release_date = read_line(lines.next(), "発売日がありません。")?;
    if !Regex::new(r"^\d{4}-\d{2}-\d{2}$")
        .unwrap()
        .is_match(release_date)
    {
        return LoadTagsError {
            message: "発売日の形式が不正です。",
        }
        .into();
    }

    let mut album_info = AlbumInfo::new(
        Some(album.to_string()),
        Some(album_artist.to_string()),
        Some(release_date.to_string()),
    );

    if read_line(lines.next(), "発売日の次の空白行がありませんでした。")?.len() > 0
    {
        return LoadTagsError {
            message: "発売日の次の行が空白行ではありません。",
        }
        .into();
    }

    let mut current_disc_info: &mut DiscInfo = album_info.new_disc();

    loop {
        let line = match lines.next() {
            Some(line) => line.trim_end(),
            None => break,
        };

        if line.len() == 0 {
            if current_disc_info.tracks().len() == 0 {
                return LoadTagsError {
                    message: "空白行が連続しています。",
                }
                .into();
            }
            current_disc_info = album_info.new_disc();
        } else {
            let (title, artists) = parse_track(line)?;
            current_disc_info.new_track(Some(title), artists);
        }
    }

    if let Some(image_filepath) = find_image_file(tags_filepath.parent().unwrap())? {
        let image_format = ImageFormat::from_filepath(image_filepath.as_path())?;
        let image_data = fs::read(image_filepath)?;
        let image = Image::new(image_format, image_data);
        album_info.set_art_work(Some(image));
    }

    Ok(album_info)
}

fn read_tags_file(tags_filepath: &Path) -> Result<String> {
    let tags_file_contents = match fs::read(tags_filepath) {
        Ok(tags_file_contents) => tags_file_contents,
        Err(_) => {
            return LoadTagsError {
                message: "tagsファイルが読み込めません",
            }
            .into()
        }
    };

    let tags_file_contents = match String::from_utf8(tags_file_contents) {
        Ok(tags_file_contents) => tags_file_contents,
        Err(_) => {
            return LoadTagsError {
                message: "tagsファイルの内容がUTF-8でエンコードされていません",
            }
            .into()
        }
    };

    Ok(tags_file_contents.nfc().collect::<String>())
}

fn read_line<'a>(line: Option<&'a str>, missing_error_message: &'static str) -> Result<&'a str> {
    match line {
        Some(line) => Ok(line.trim()),
        None => {
            return LoadTagsError {
                message: missing_error_message,
            }
            .into()
        }
    }
}

/// トラックの行をパースする。
fn parse_track(line: &str) -> Result<(String, Vec<String>)> {
    let mut split_line = line.split("//");

    let title = match split_line.next() {
        Some(title) => title,
        None => {
            return LoadTagsError {
                message: "タイトルがありません。",
            }
            .into()
        }
    };

    let mut artists = vec![];
    for artist in split_line {
        artists.push(artist.to_string());
    }

    Ok((title.to_string(), artists))
}

/// 画像ファイルを探す。
fn find_image_file(folder: &Path) -> Result<Option<PathBuf>> {
    for dir_entry in folder.read_dir()? {
        let filepath = dir_entry?.path();
        if ImageFormat::is_image_file(filepath.as_path()) {
            return Ok(Some(filepath));
        }
    }
    return Ok(None);
}

/// tagsファイルを出力する。
pub fn write_tags_file(tags_filepath: &Path, album_info: &AlbumInfo) -> Result<()> {
    let mut s = String::new();

    s.push_str(album_info.album().unwrap_or(""));
    s.push('\n');

    s.push_str(album_info.album_artist().unwrap_or(""));
    s.push('\n');

    s.push_str(album_info.release_date().unwrap_or(""));
    s.push('\n');

    for disc_info in album_info.discs() {
        s.push('\n');

        for track_info in disc_info.tracks() {
            s.push_str(track_info.title().unwrap_or(""));

            for artist in track_info.artists() {
                s.push_str("//");
                s.push_str(artist);
            }

            s.push('\n');
        }
    }

    let nfc = s.nfc().collect::<String>();

    if let Err(_) = fs::write(tags_filepath, &nfc) {
        return WriteTagsError {
            message: "tagsファイルが書き込めませんでした。",
        }
        .into();
    };

    Ok(())
}

/// 画像をファイルに出力する。
pub fn write_art_work_file(art_work: Option<&Image>) -> Result<()> {
    let art_work = match art_work {
        Some(art_work) => art_work,
        None => return Ok(()),
    };

    let art_work_filepath = format!("Folder.{}", art_work.format().extension());
    let art_work_filepath = Path::new(art_work_filepath.as_str());

    fs::write(art_work_filepath, art_work.data())?;

    Ok(())
}

/// tags読み込みエラー
#[derive(Debug, Clone)]
pub struct LoadTagsError {
    message: &'static str,
}

impl fmt::Display for LoadTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.message)
    }
}

impl std::error::Error for LoadTagsError {}

impl From<LoadTagsError> for Result<(String, Vec<String>)> {
    fn from(error: LoadTagsError) -> Self {
        Err(Box::new(error))
    }
}

impl From<LoadTagsError> for Result<&str> {
    fn from(error: LoadTagsError) -> Self {
        Err(Box::new(error))
    }
}

impl From<LoadTagsError> for Result<String> {
    fn from(error: LoadTagsError) -> Self {
        Err(Box::new(error))
    }
}

impl From<LoadTagsError> for Result<AlbumInfo> {
    fn from(error: LoadTagsError) -> Self {
        Err(Box::new(error))
    }
}

/// tags書き込みエラー
#[derive(Debug, Clone)]
pub struct WriteTagsError {
    message: &'static str,
}

impl fmt::Display for WriteTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.message)
    }
}

impl std::error::Error for WriteTagsError {}

impl From<WriteTagsError> for Result<()> {
    fn from(error: WriteTagsError) -> Self {
        Err(Box::new(error))
    }
}
