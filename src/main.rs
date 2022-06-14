mod tags_file;
mod tags;
mod flow;
mod audio_file;
mod common;
mod model;

fn main() -> common::Result<()> {
    flow::execute()?;

    Ok(())
}
