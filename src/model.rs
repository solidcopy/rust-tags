use crate::tags::Image;

/// アルバム情報
#[derive(Debug)]
pub struct AlbumInfo {
    album: Option<String>,
    album_artist: Option<String>,
    release_date: Option<String>,
    discs: Vec<DiscInfo>,
    art_work: Option<Image>,
}

impl AlbumInfo {
    /// アルバム情報を作成する。
    pub fn new(
        album: Option<String>,
        album_artist: Option<String>,
        release_date: Option<String>,
    ) -> AlbumInfo {
        AlbumInfo {
            album,
            album_artist,
            release_date,
            discs: Vec::with_capacity(1),
            art_work: None,
        }
    }

    pub fn album(&self) -> Option<&str> {
        match &self.album {
            Some(album) => Some(album.as_str()),
            None => None,
        }
    }

    pub fn album_artist(&self) -> Option<&str> {
        match &self.album_artist {
            Some(album_artist) => Some(album_artist.as_str()),
            None => None,
        }
    }

    pub fn release_date(&self) -> Option<&str> {
        match &self.release_date {
            Some(release_date) => Some(release_date.as_str()),
            None => None,
        }
    }

    pub fn discs(&self) -> &Vec<DiscInfo> {
        &self.discs
    }

    pub fn new_disc(&mut self) -> &mut DiscInfo {
        let disc = DiscInfo { tracks: vec![] };
        self.discs.push(disc);
        self.discs.last_mut().unwrap()
    }

    pub fn art_work(&self) -> Option<&Image> {
        match &self.art_work {
            Some(art_work) => Some(art_work),
            None => None,
        }
    }

    pub fn set_art_work(&mut self, art_work: Option<Image>) {
        self.art_work = art_work;
    }
}

/// ディスク情報
#[derive(Debug)]
pub struct DiscInfo {
    tracks: Vec<TrackInfo>,
}

impl DiscInfo {
    pub fn tracks(&self) -> &Vec<TrackInfo> {
        &self.tracks
    }

    pub fn new_track(&mut self, title: Option<String>, artists: Vec<String>) -> &mut TrackInfo {
        let track_info = TrackInfo { title, artists };
        self.tracks.push(track_info);
        self.tracks.last_mut().unwrap()
    }
}

/// トラック情報
#[derive(Debug)]
pub struct TrackInfo {
    title: Option<String>,
    artists: Vec<String>,
}

impl TrackInfo {
    pub fn title(&self) -> Option<&str> {
        match &self.title {
            Some(title) => Some(title.as_str()),
            None => None,
        }
    }

    pub fn artists(&self) -> &Vec<String> {
        &self.artists
    }
}
