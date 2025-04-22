mod compile_mir;
mod parser;
mod sampler;
mod writer;

use colored::Colorize;
use compile_mir::compile_mir;
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
    let generations: Vec<PathBuf> = glob(&pattern)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .filter(|path| path.is_file() && path.extension() == Some(OsStr::new("json")))
        // .take(1) // debugging
        .collect();

    // Parses the vLLM JSON generation
    // Samples a correct generation per problem
    // Writes it out to `bin` path
    let binaries: Vec<String> = generations
        .iter()
        .filter_map(|path| {
            let json = parser::parse_gen(path).ok()?;
            let (problem_name, generation) = sampler::sample_gen(&json).ok()?;
            writer::write_bin(problem_name, generation).ok()
        })
        .collect();

    println!("generations: {}", generations.len());
    println!("binaries: {}", binaries.len());

    // Compiles MIR
    // mir_files contains the mapping between ./bin/script.rs and ./mir/script.nll.--.mir
    let out_clean_mir = Path::new("./mir/");
    let mir_files: Vec<(String, String)> = binaries
        .iter()
        .filter_map(|binpath| compile_mir(binpath, out_clean_mir).ok())
        .collect();

    // Parse the mir and map it to the binary LOC
    show(&mir_files);
}

fn show(tuple_files: &Vec<(String, String)>) {
    for (bin, mir) in tuple_files {
        println!("{} - {}", bin, mir);
    }
}
