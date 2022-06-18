use std::fmt;
use std::fmt::{Debug, Formatter};
use std::path::Path;

use imghdr;

use crate::common::Result;
use crate::model::{AlbumInfo, DiscInfo, TrackInfo};
use crate::tags::flac_tag_io::FlacIOImpl;

pub mod flac_tag_io;

/// タグIO
///
/// タグの参照/設定を行う。
pub trait TagIO {
    fn load(&self, filepath: &Path) -> Result<Tags>;
    fn save(&self, filepath: &Path, tags: &Tags) -> Result<()>;
}

/// 指定されたファイルの形式に対応するタグIO実装を返す。
pub fn tag_io_for(filepath: &Path) -> Option<Box<dyn TagIO>> {
    let extension = match filepath.extension() {
        Some(extension) => extension.to_str().unwrap(),
        None => return None,
    };
    match extension {
        "flac" => Some(Box::new(FlacIOImpl)),
        _ => None
    }
}

/// タグ情報
pub struct Tags {
    album: Option<String>,
    album_artist: Option<String>,
    release_date: Option<String>,
    art_work: Option<Image>,
    number_of_discs: Option<usize>,
    disc_number: Option<usize>,
    number_of_tracks: Option<usize>,
    track_number: Option<usize>,
    title: Option<String>,
    artists: Vec<String>,
}

impl Tags {
    pub fn new() -> Tags {
        Tags {
            album: None,
            album_artist: None,
            release_date: None,
            art_work: None,
            number_of_discs: None,
            disc_number: None,
            number_of_tracks: None,
            track_number: None,
            title: None,
            artists: vec![],
        }
    }

    pub fn album(&self) -> Option<&str> {
        self.album.as_ref().map(|s| s.as_str())
    }

    pub fn set_album(&mut self, album: Option<String>) {
        self.album = album;
    }

    pub fn album_artist(&self) -> Option<&str> {
        self.album_artist.as_ref().map(|s| s.as_str())
    }

    pub fn set_album_artist(&mut self, album_artist: Option<String>) {
        self.album_artist = album_artist;
    }

    pub fn release_date(&self) -> Option<&str> {
        self.release_date.as_ref().map(|s| s.as_str())
    }

    pub fn set_release_date(&mut self, release_date: Option<String>) {
        self.release_date = release_date;
    }

    pub fn art_work(&self) -> Option<&Image> {
        self.art_work.as_ref()
    }

    pub fn set_art_work(&mut self, art_work: Option<Image>) {
        self.art_work = art_work;
    }

    pub fn number_of_discs(&self) -> Option<usize> {
        self.number_of_discs
    }

    pub fn set_number_of_discs(&mut self, number_of_discs: Option<usize>) {
        self.number_of_discs = number_of_discs;
    }

    pub fn disc_number(&self) -> Option<usize> {
        self.disc_number
    }

    pub fn set_disc_number(&mut self, disc_number: Option<usize>) {
        self.disc_number = disc_number;
    }

    pub fn number_of_tracks(&self) -> Option<usize> {
        self.number_of_tracks
    }

    pub fn set_number_of_tracks(&mut self, number_of_tracks: Option<usize>) {
        self.number_of_tracks = number_of_tracks;
    }

    pub fn track_number(&self) -> Option<usize> {
        self.track_number
    }

    pub fn set_track_number(&mut self, track_number: Option<usize>) {
        self.track_number = track_number;
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_ref().map(|s| s.as_str())
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    pub fn artists(&self) -> &Vec<String> {
        &self.artists
    }

    pub fn add_artist(&mut self, artist: String) {
        self.artists.push(artist);
    }

    /// アルバム情報でタグ情報を更新する。
    pub fn update_album_info(&mut self, album_info: &AlbumInfo) {
        self.set_album(album_info.album().map(String::from));
        self.set_album_artist(album_info.album_artist().map(String::from));
        self.set_release_date(album_info.release_date().map(String::from));
        match album_info.art_work() {
            Some(image) => self.set_art_work(Some(image.clone())),
            None => self.set_art_work(None),
        }
        self.set_number_of_discs(Some(album_info.discs().len()));
    }

    /// ディスク情報でタグ情報を更新する。
    pub fn update_disc_info(&mut self, disc_number: usize, disc_info: &DiscInfo) {
        self.set_disc_number(Some(disc_number));
        self.set_number_of_tracks(Some(disc_info.tracks().len()));
    }

    /// トラック情報でタグ情報を更新する。
    pub fn update_track_info(&mut self, track_number: usize, track_info: &TrackInfo) {
        self.set_track_number(Some(track_number));
        self.set_title(track_info.title().map(String::from));
        for artist in track_info.artists() {
            self.add_artist(artist.clone());
        }
    }
}

/// 画像フォーマット
#[derive(Clone, Copy)]
pub enum ImageFormat {
    JPEG,
    PNG,
}

impl ImageFormat {
    /// この画像フォーマットのMIMEタイプを返す。
    pub fn mime(&self) -> &'static str {
        match self {
            ImageFormat::JPEG => "image/jpeg",
            ImageFormat::PNG => "image/png",
        }
    }

    /// この画像フォーマットのファイル拡張子を返す。
    pub fn extension(&self) -> &'static str {
        match self {
            ImageFormat::JPEG => "jpg",
            ImageFormat::PNG => "png",
        }
    }

    /// 指定されたデータの内容から画像フォーマットを判定する。
    pub fn from_data(image_data: &Vec<u8>) -> Result<ImageFormat> {
        match imghdr::from_bytes(image_data) {
            Some(imghdr::Type::Jpeg) => Ok(ImageFormat::JPEG),
            Some(imghdr::Type::Png) => Ok(ImageFormat::PNG),
            _ => ImageFormatError.into(),
        }
    }

    /// 指定されたファイルパスから画像フォーマットを判定する。
    pub fn from_filepath(filepath: &Path) -> Result<ImageFormat> {
        if let Some(extension) = filepath.extension() {
            let extension = extension.to_str().unwrap().to_lowercase();
            match extension.as_str() {
                "jpg" => Ok(ImageFormat::JPEG),
                "jpeg" => Ok(ImageFormat::JPEG),
                "png" => Ok(ImageFormat::PNG),
                _ => ImageFormatError.into(),
            }
        } else {
            ImageFormatError.into()
        }
    }

    /// 指定されたファイルが画像ファイルであるかを判定する。
    pub fn is_image_file(filepath: &Path) -> bool {
        match filepath.extension() {
            Some(extension) => {
                let extension = extension.to_str().unwrap();
                for image_extension in ["jpg", "png"] {
                    if extension == image_extension {
                        return true;
                    }
                }
                false
            }
            None => false
        }
    }
}

/// 画像
#[derive(Clone)]
pub struct Image {
    format: ImageFormat,
    data: Vec<u8>,
}

impl Image {
    /// 画像を作成する。
    pub fn new(format: ImageFormat, data: Vec<u8>) -> Image {
        Image { format, data }
    }

    pub fn format(&self) -> ImageFormat {
        self.format
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<Image mime=\"{}\" size=\"{}\"", self.format.mime(), self.data.len())
    }
}

/// 画像フォーマットエラー
///
/// 非対応の形式である、または画像ファイルでないデータやファイルを画像として処理しようとすると発生する。
#[derive(Debug, Clone)]
pub struct ImageFormatError;

impl fmt::Display for ImageFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "画像のMIMEタイプが不正です")
    }
}

impl std::error::Error for ImageFormatError {}

impl From<ImageFormatError> for Result<ImageFormat> {
    fn from(error: ImageFormatError) -> Self {
        Err(Box::new(error))
    }
}