mod parser;
mod sampler;
mod writer;

use colored::Colorize;
use glob::glob;
use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
};

fn main() {
    let dir = env::args().nth(1).unwrap_or_else(|| {
        panic!(
            "{} Incorrect Usage.\nPlease provide a directory of scipts: `cargo run -- data/multipl-e/`",
            "[ERROR]".red()
        )
    });

    let pattern: String = Path::new(&dir)
        .join("*")
        .to_str()
        .expect("Invalid UTF-8 path")
        .to_owned();

    println!("Loading generations from `{}`...", pattern);

    // TODO: Re-organize this in a better way
    // TODO: Use Options instead of Results
    let json_gens: Vec<PathBuf> = glob(&pattern)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .filter(|path| path.is_file() && path.extension() == Some(OsStr::new("json")))
        .collect();

    // Parse the vLLM JSON generation
    // Sample a correct generation per problem
    // Write it out to `bin` path
    let binary_paths: Vec<String> = json_gens
        .iter()
        .filter_map(|path| {
            let json = parser::parse_gen(path).ok()?;
            let (problem_name, generation) = sampler::sample_gen(&json).ok()?;
            writer::write_bin(problem_name, generation).ok()
        })
        .collect();

    println!("json_gens: {}", json_gens.len());
    println!("binaries: {}", binary_paths.len());
}
