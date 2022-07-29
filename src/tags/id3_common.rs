use id3::frame::PictureType;
use id3::{Tag, TagLike};

use crate::tags::{Image, ImageFormat, Tags};
use anyhow::Result;

pub fn load_id3(file_tags: &Tag) -> Result<Tags> {
    let mut tags = Tags::new();

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
