mod audio_file;
mod common;
mod flow;
mod model;
mod tags;
mod tags_file;
use anyhow::Result;

fn main() -> Result<()> {
    flow::execute()?;

    Ok(())
}
