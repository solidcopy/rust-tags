use std::path::Path;

use dsf::DsfFile;

use crate::common::Result;
use crate::tags::id3_common::load_id3;
use crate::tags::{TagIO, Tags};

/// ID3タグIO実装
pub struct DsfIOImpl;

impl TagIO for DsfIOImpl {
    fn load(&self, filepath: &Path) -> Result<Tags> {
        let dsf_file = DsfFile::open(filepath)?;
        let file_tags = match dsf_file.id3_tag() {
            Some(file_tags) => file_tags,
            None => return Ok(Tags::new()),
        };

        load_id3(file_tags)
    }

    fn save(&self, _: &Path, _: &Tags) -> Result<()> {
        panic!("dsfクレートがまだ書き込みに対応していない");
    }
}
