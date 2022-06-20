use std::path::Path;

use mp4ameta::{Img, ImgFmt, Tag};

use crate::common::Result;
use crate::tags::{Image, ImageFormat, ImageFormatError, TagIO, Tags};

/// M4AタグIO実装
pub struct M4AIOImpl;

impl TagIO for M4AIOImpl {
    fn load(&self, filepath: &Path) -> Result<Tags> {
        let mut file_tags = Tag::read_from_path(filepath)?;

        let mut tags = Tags::new();

        tags.set_album(file_tags.take_album());
        tags.set_album_artist(file_tags.take_album_artist());
        tags.set_release_date(file_tags.take_year());
        tags.set_number_of_discs(file_tags.total_discs().map(|n| n as usize));
        tags.set_disc_number(file_tags.disc_number().map(|n| n as usize));
        tags.set_number_of_tracks(file_tags.total_tracks().map(|n| n as usize));
        tags.set_track_number(file_tags.track_number().map(|n| n as usize));
        tags.set_title(file_tags.take_title());

        for artist in file_tags.take_artists() {
            tags.add_artist(artist);
        }

        if let Some(art_work) = file_tags.take_artwork() {
            let format = to_common_image_format(&art_work.fmt)?;
            let image = Image {
                format,
                data: art_work.data,
            };
            tags.set_art_work(Some(image));
        }

        Ok(tags)
    }

    fn save(&self, filepath: &Path, tags: &Tags) -> Result<()> {
        let mut file_tags = Tag::read_from_path(filepath)?;
        file_tags.clear();

        if let Some(album) = tags.album() {
            file_tags.set_album(album);
        }
        if let Some(album_artist) = tags.album_artist() {
            file_tags.set_album_artist(album_artist);
        }
        if let Some(release_date) = tags.release_date() {
            file_tags.set_year(release_date);
        }
        if let Some(number_of_discs) = tags.number_of_discs() {
            file_tags.set_total_discs(number_of_discs as u16);
        }
        if let Some(disc_number) = tags.disc_number() {
            file_tags.set_disc_number(disc_number as u16);
        }
        if let Some(number_of_tracks) = tags.number_of_tracks() {
            file_tags.set_total_tracks(number_of_tracks as u16);
        }
        if let Some(track_number) = tags.track_number() {
            file_tags.set_track_number(track_number as u16);
        }
        if let Some(title) = tags.title() {
            file_tags.set_title(title);
        }
        file_tags.set_artists(tags.artists.clone());

        if let Some(image) = tags.art_work() {
            let format = to_m4a_image_format(&image.format)?;
            let data = image.data.to_owned();
            let art_work = Img::new(format, data);
            file_tags.add_artwork(art_work);
        }

        file_tags.write_to_path(filepath)?;

        Ok(())
    }
}

fn to_common_image_format(m4a_image_format: &ImgFmt) -> Result<ImageFormat> {
    match m4a_image_format {
        ImgFmt::Jpeg => Ok(ImageFormat::JPEG),
        ImgFmt::Png => Ok(ImageFormat::PNG),
        _ => ImageFormatError.into(),
    }
}

fn to_m4a_image_format(format: &ImageFormat) -> Result<ImgFmt> {
    match format {
        ImageFormat::JPEG => Ok(ImgFmt::Jpeg),
        ImageFormat::PNG => Ok(ImgFmt::Png),
    }
}
