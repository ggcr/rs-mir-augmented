use anyhow::Result;
use colored::Colorize;
use glob::glob;
use polars::prelude::*;
use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
};

fn sample_correct_gen(sample: &Path) -> Result<String> {
    // Parse the JSON
    let mut file = std::fs::File::open(sample).unwrap();
    let df = JsonReader::new(&mut file).finish()?;
    // Unwrap struct and check for status == "OK"
    let n_generations = df
        .clone()
        .lazy()
        .select([(col("results"))])
        .explode(["results"])
        .unnest(["results"])
        .filter(col("status").eq(lit("OK")))
        .limit(1)
        .collect()?;
    // Sample a single correct gen (Status == "OK")
    let program = n_generations.column("program")?.get(0)?;
    Ok(program.to_string())
}

fn main() {
    let dir = env::args()
        .nth(1)
        .unwrap_or_else(|err| panic!("{} Incorrect Usage.\nPlease provide a directory path of the generations `cargo run -- data/multipl-e/`", "[ERROR]".red()));

    let pattern: String = Path::new(&dir)
        .join("*")
        .to_str()
        .expect("Not a valid UTF-8 path")
        .to_owned();

    println!("Loading generations for `{}`...", pattern);

    // Glob for all files in dir
    let json_gens: Vec<PathBuf> = glob(&pattern)
        .expect("Failed to read glob pattern")
        .filter_map(|path| path.ok()) // pattern matches
        .filter(|path| path.is_file() && path.extension() == Some(OsStr::new("json")))
        .collect();

    // Sample 1 random correct gen for each problem
    let correct_gens: Vec<String> = json_gens
        .iter()
        .filter_map(|sample| sample_correct_gen(sample).ok())
        .collect();

    println!("json_gens: {}", json_gens.len());
    println!("correct_gens: {}", correct_gens.len());
}
