use std::{fs, path::Path};

use anyhow::Result;

pub fn write_bin(name: String, program: String) -> Result<String> {
    // Up until here the generations have been parsed and we sampled the ones we want.
    // In here we will write each correct sampled generation onto the `bin` dir.

    // Create binary path as: projroot/src/bin/<name>.rs
    let bin_path = Path::new("./bin/").join(name.clone() + ".rs");
    if let Some(parent) = bin_path.parent() {
        fs::create_dir_all(parent)?; // src/bin/
    }
    fs::write(&bin_path, program)?;

    // If success we return the path of the binary file
    Ok(bin_path.to_string_lossy().to_string())
}
