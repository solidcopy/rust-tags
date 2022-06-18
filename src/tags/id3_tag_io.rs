use std::path::Path;
use std::str::FromStr;

use id3::{Tag, TagLike, Timestamp, Version};
use id3::frame::{Picture, PictureType};

use crate::common::Result;
use crate::tags::{Image, ImageFormat, TagIO, Tags};

/// ID3タグIO実装
pub struct ID3IOImpl;

impl TagIO for ID3IOImpl {
    fn load(&self, filepath: &Path) -> Result<Tags> {
        let mut tags = Tags::new();

        let file_tags = Tag::read_from_path(filepath)?;

        tags.set_album(file_tags.album().map(String::from));
        tags.set_album_artist(file_tags.album_artist().map(String::from));
        if let Some(ts) = file_tags.date_released() {
            if let (Some(month), Some(day)) = (ts.month, ts.day) {
                let release_date = format!("{:04}-{:02}-{:02}", ts.year, month, day);
                tags.set_release_date(Some(release_date));
            }
        }
        tags.set_number_of_discs(file_tags.total_discs().map(|n| n as usize));
        tags.set_disc_number(file_tags.disc().map(|n| n as usize));
        tags.set_number_of_tracks(file_tags.total_tracks().map(|n| n as usize));
        tags.set_track_number(file_tags.track().map(|n| n as usize));
        tags.set_title(file_tags.title().map(String::from));

        if let Some(artists) = file_tags.artist() {
            // アーティストは複数あっても1つのタグになっている
            // 区切り文字は明確に決まっていないので、\0 ; \\ のいずれかと想定する
            for artist in artists.split(['\0', ';']) {
                for artist in artist.split("\\\\") {
                    tags.add_artist(artist.to_string());
                }
            }
        }

        for picture in file_tags.pictures() {
            if picture.picture_type == PictureType::CoverFront {
                let format = ImageFormat::from_data(&picture.data)?;
                let data = picture.data.to_owned();
                let image = Some(Image { format, data });
                tags.set_art_work(image);
                break;
            }
        }

        Ok(tags)
    }

    fn save(&self, filepath: &Path, tags: &Tags) -> Result<()> {
        let mut file_tags = Tag::new();

        if let Some(album) = tags.album() {
            file_tags.set_album(album);
        }
        if let Some(album_artist) = tags.album_artist() {
            file_tags.set_album_artist(album_artist);
        }
        if let Some(release_date) = tags.release_date() {
            let release_date = Timestamp::from_str(release_date)?;
            file_tags.set_date_released(release_date);
        }
        if let Some(number_of_discs) = tags.number_of_discs() {
            file_tags.set_total_discs(number_of_discs as u32);
        }
        if let Some(disc_number) = tags.disc_number() {
            file_tags.set_disc(disc_number as u32);
        }
        if let Some(number_of_tracks) = tags.number_of_tracks() {
            file_tags.set_total_tracks(number_of_tracks as u32);
        }
        if let Some(track_number) = tags.track_number() {
            file_tags.set_track(track_number as u32);
        }
        if let Some(title) = tags.title() {
            file_tags.set_title(title);
        }

        let mut artists = String::new();
        for artist in tags.artists() {
            if artists.len() > 0 {
                artists.push(';');
            }
            artists.push_str(artist);
        }
        if artists.len() > 0 {
            file_tags.set_artist(artists);
        }

        if let Some(image) = tags.art_work() {
            let mime_type = image.format.mime().to_string();
            let data = image.data.to_owned();
            file_tags.add_frame(Picture { mime_type, picture_type: PictureType::CoverFront, description: String::new(), data });
        }

        file_tags.write_to_path(filepath, Version::Id3v24)?;

        Ok(())
    }
}
