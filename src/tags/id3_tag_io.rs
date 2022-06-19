use std::path::Path;
use std::str::FromStr;

use id3::frame::{Picture, PictureType};
use id3::{Tag, TagLike, Timestamp, Version};

use crate::common::Result;
use crate::tags::id3_common::load_id3;
use crate::tags::{TagIO, Tags};

/// ID3タグIO実装
pub struct ID3IOImpl;

impl TagIO for ID3IOImpl {
    fn load(&self, filepath: &Path) -> Result<Tags> {
        let file_tags = Tag::read_from_path(filepath)?;

        load_id3(&file_tags)
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
            file_tags.add_frame(Picture {
                mime_type,
                picture_type: PictureType::CoverFront,
                description: String::new(),
                data,
            });
        }

        file_tags.write_to_path(filepath, Version::Id3v24)?;

        Ok(())
    }
}
