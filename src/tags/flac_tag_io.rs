use std::path::Path;
use std::str::FromStr;

use metaflac::block::PictureType;
use metaflac::{BlockType, Tag};

use crate::tags::{Image, ImageFormat, TagIO, Tags};
use anyhow::Result;

/// FLACタグIO実装
pub struct FlacIOImpl;

impl TagIO for FlacIOImpl {
    fn load(&self, filepath: &Path) -> Result<Tags> {
        let mut tags = Tags::new();

        let file_tags = Tag::read_from_path(filepath)?;
        tags.set_album(get_string(&file_tags, "ALBUM").map(String::from));
        tags.set_album_artist(get_string(&file_tags, "ALBUMARTIST").map(String::from));
        tags.set_release_date(get_string(&file_tags, "DATE").map(String::from));
        tags.set_number_of_discs(get_usize(&file_tags, "DISCTOTAL"));
        tags.set_disc_number(get_usize(&file_tags, "DISCNUMBER"));
        tags.set_number_of_tracks(get_usize(&file_tags, "TRACKTOTAL"));
        tags.set_track_number(get_usize(&file_tags, "TRACKNUMBER"));
        tags.set_title(get_string(&file_tags, "TITLE").map(String::from));
        for artist in get_str_vec(&file_tags, "ARTIST") {
            tags.add_artist(artist.to_string());
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
        let mut file_tag = Tag::read_from_path(filepath)?;

        file_tag.remove_blocks(BlockType::VorbisComment);
        file_tag.remove_blocks(BlockType::Picture);

        set_string(&mut file_tag, "ALBUM", tags.album());
        set_string(&mut file_tag, "ALBUMARTIST", tags.album_artist());
        set_string(&mut file_tag, "DATE", tags.release_date());
        set_usize(&mut file_tag, "DISCTOTAL", tags.number_of_discs());
        set_usize(&mut file_tag, "DISCNUMBER", tags.disc_number());
        set_usize(&mut file_tag, "TRACKTOTAL", tags.number_of_tracks());
        set_usize(&mut file_tag, "TRACKNUMBER", tags.track_number());
        set_string(&mut file_tag, "TITLE", tags.title());
        file_tag.set_vorbis("ARTIST", tags.artists.clone());
        set_picture(&mut file_tag, PictureType::CoverFront, tags.art_work());

        file_tag.write_to_path(filepath)?;

        Ok(())
    }
}

fn get_string<'a>(tags: &'a Tag, item_name: &str) -> Option<&'a str> {
    match tags.get_vorbis(item_name) {
        Some(mut value) => Some(value.next().unwrap()),
        None => None,
    }
}

fn set_string(file_tag: &mut Tag, tag_name: &str, s: Option<&str>) {
    if let Some(s) = s {
        file_tag.set_vorbis(tag_name, vec![s]);
    }
}

fn get_usize(tags: &Tag, item_name: &str) -> Option<usize> {
    match get_string(tags, item_name) {
        Some(value) => Some(usize::from_str(value).unwrap()),
        None => None,
    }
}

fn set_usize(file_tag: &mut Tag, tag_name: &str, n: Option<usize>) {
    if let Some(s) = n {
        file_tag.set_vorbis(tag_name, vec![s.to_string()]);
    }
}

fn get_str_vec<'a>(tags: &'a Tag, item_name: &str) -> Vec<&'a str> {
    let mut vec = vec![];

    if let Some(values) = tags.get_vorbis(item_name) {
        for value in values {
            vec.push(value);
        }
    }

    vec
}

fn set_picture(file_tag: &mut Tag, picture_type: PictureType, image: Option<&Image>) {
    if let Some(image) = image {
        file_tag.add_picture(image.format().mime(), picture_type, image.data().clone());
    }
}
