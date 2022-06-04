mod tags_file;

use std::path::Path;

fn main() {
    let tags_filepath = Path::new("I:\\music\\物語シリーズ\\歌物語 -物語シリーズ主題歌集-\\tags");
    let package = tags_file::load_package(tags_filepath);

    tags_file::write_tags_file(Path::new("tags"), &package);
    println!("completed");
}
