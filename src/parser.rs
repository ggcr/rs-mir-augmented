use anyhow::Result;
use polars::{frame::DataFrame, prelude::*};
use std::path::Path;

pub fn parse_gen(path: &Path) -> Result<DataFrame> {
    // Parse the JSON
    let mut file = std::fs::File::open(path).unwrap();
    let df = JsonReader::new(&mut file).finish()?;
    Ok(df)
}
