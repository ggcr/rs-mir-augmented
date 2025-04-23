use colored::Colorize;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{Result, anyhow};

// TODO: Define an enum for our own custom error type
// Success, Warning, Error

// TODO: Binary should be a PathBuf
pub fn compile_mir(binary: &String, dest_dir: &Path) -> Result<(String, String)> {
    // rustc -Z dump-mir=all ./src/bin/<binary>.rs
    let status = Command::new("rustc")
        .arg("-Z")
        .arg("dump-mir=all")
        .arg(binary)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if !status.success() {
        // warning
        return Err(anyhow!("{}: {}", "[COMPILE FAILED]".bright_red(), binary));
    }
    println!("{}: {}", "[COMPILED]".green(), binary);

    // retrieve the *.nll.-------------.mir file
    // pattern: ./mir_dump/<binary file name>.*(anything that is not "main").-------.nll.0.mir
    // we ignore the mir for the main as it only contains asserts
    let mir_dir = Path::new("mir_dump");
    let mir_path = find_non_main_mir(mir_dir)?
        .ok_or_else(|| anyhow!("{}: {}", "[MIR NOT FOUND]".red(), binary))?;

    // mv mir_path to dest_dir/.
    fs::create_dir_all(dest_dir)?;
    let dest_path = dest_dir.join(mir_path.file_name().unwrap());
    fs::rename(&mir_path, &dest_path)?;
    println!("{}: {}", "[MIR FILE]".green(), dest_path.display());

    // empty mir_path dir
    fs::remove_dir_all(mir_dir).unwrap();
    fs::create_dir(mir_dir).unwrap();

    Ok((binary.to_owned(), dest_path.to_string_lossy().to_string()))
}

fn find_non_main_mir(dir: &Path) -> Result<Option<PathBuf>> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        let filename = path.file_name().unwrap().to_string_lossy();
        if !filename.contains("main") && filename.contains(".nll.") && filename.ends_with(".mir") {
            return Ok(Some(path));
        }
    }
    Ok(None)
}
