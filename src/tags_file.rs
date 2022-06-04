use std::fs;
use std::path::Path;

use chrono::naive::NaiveDate;

/// パッケージ
#[derive(Debug)]
pub struct Package {
    album: String,
    album_artist: String,
    release_date: NaiveDate,
    discs: Vec<Disc>,
}

impl Package {
    pub fn new(album: String, album_artist: String, release_date: NaiveDate) -> Package {
        Package { album, album_artist, release_date, discs: Vec::with_capacity(1) }
    }

    pub fn album(&self) -> &str {
        &self.album
    }

    pub fn album_artist(&self) -> &str {
        &self.album_artist
    }

    pub fn release_date(&self) -> &NaiveDate {
        &self.release_date
    }

    pub fn discs(&self) -> &Vec<Disc> {
        &self.discs
    }

    pub fn new_disc(&mut self) -> &mut Disc {
        let mut disc = Disc { number: self.discs.len() + 1, tracks: vec![] };
        disc.number = self.discs.len() + 1;
        self.discs.push(disc);
        self.discs.last_mut().unwrap()
    }
}

/// ディスク
#[derive(Debug)]
pub struct Disc {
    number: usize,
    tracks: Vec<Track>,
}

impl Disc {
    pub fn number(&self) -> usize {
        self.number
    }

    pub fn tracks(&self) -> &Vec<Track> {
        &self.tracks
    }

    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }
}

/// トラック
#[derive(Debug)]
pub struct Track {
    title: String,
    artists: Vec<String>,
}

impl Track {
    pub fn new(title: String) -> Track {
        Track { title, artists: vec![] }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn artists(&self) -> &Vec<String> {
        &self.artists
    }

    pub fn add_artist(&mut self, artist: String) {
        self.artists.push(artist);
    }
}

/// tagsファイルを読み込んでパッケージ情報を作成する。
pub fn load_package(tags_filepath: &Path) -> Package {
    let tags_file_contents = fs::read(tags_filepath).expect("tagsファイルが読み込めませんでした。");
    let tags_file_contents = String::from_utf8(tags_file_contents).expect("tagsファイルの内容がUTF-8でデコードできませんでした。");


    let mut lines = tags_file_contents.lines();

    let album = lines.next().expect("アルバム名がありません。").trim_end();
    let album_artist = lines.next().expect("アルバムアーティスト名がありません。").trim_end();
    let release_date = lines.next().expect("発売日がありません。").trim_end();
    let release_date = NaiveDate::parse_from_str(&release_date, "%Y-%m-%d").expect("発売日の形式が不正です。");
    let mut package = Package::new(album.to_string(), album_artist.to_string(), release_date);

    if lines.next().expect("発売日の次の空白行がありませんでした。").trim_end().len() > 0 {
        panic!("発売日の次の行が空白行ではありません。");
    }

    let mut current_disc: &mut Disc = package.new_disc();

    loop {
        let line = match lines.next() {
            Some(line) => line.trim_end(),
            None => break,
        };

        if line.len() == 0 {
            if current_disc.tracks().len() == 0 {
                panic!("空白行が連続しています。");
            }
            current_disc = package.new_disc();
        } else {
            let track = parse_track(line);
            current_disc.add_track(track);
        }
    }

    package
}

/// トラックの行をパースする。
fn parse_track(line: &str) -> Track {
    let mut split_line = line.split("//");

    let title = split_line.next().expect("タイトルがありません。");
    let mut track = Track::new(title.to_string());

    for artist in split_line {
        track.add_artist(artist.to_string());
    }

    track
}

/// tagsファイルを出力する。
pub fn write_tags_file(tags_filepath:&Path, package:&Package) {
    let mut s = String::new();

    s.push_str(package.album());
    s.push('\n');

    s.push_str(package.album_artist());
    s.push('\n');

    s.push_str(package.release_date.format("%Y-%m-%d").to_string().as_str());
    s.push('\n');

    for disc in package.discs() {
        s.push('\n');

        for track in disc.tracks() {
            s.push_str(track.title());

            for artist in track.artists() {
                s.push_str("//");
                s.push_str(artist);
            }

            s.push('\n');
        }
    }

    fs::write(tags_filepath, &s).expect("tagsファイルが書き込めませんでした。");
}
