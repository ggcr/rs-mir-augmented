use anyhow::Result;
use polars::{frame::DataFrame, prelude::*};
use regex::Regex;
use std::{collections::HashMap, fs::read_to_string, path::Path};

pub fn parse_gen(path: &Path) -> Option<DataFrame> {
    // Parse the JSON
    let mut file = std::fs::File::open(path).unwrap();
    let df = JsonReader::new(&mut file).finish().ok()?;
    Some(df)
}

pub fn parse_mir(mir_file: &String) -> HashMap<i32, Vec<String>> {
    let mut dict: HashMap<i32, Vec<String>> = HashMap::new();

    for line in read_to_string(mir_file).unwrap().lines() {
        if !line.starts_with("| ") && line.contains("at ./bin/") {
            let statement: Vec<&str> = line.split("//").collect();
            let ins = statement.first().copied().unwrap_or("").trim().to_string();
            let comment = statement.last().copied();

            if let Some((start_loc, end_loc)) = get_loc_range(comment) {
                let loc_range: Vec<i32> = (start_loc..=end_loc).collect();

                for loc in &loc_range {
                    let mut annotated_ins = ins.clone();
                    if loc_range.len() > 1 {
                        annotated_ins.push_str(&format!(
                            "  (note: associated also with loc={:?})",
                            loc_range
                        ));
                    }
                    dict.entry(*loc).or_default().push(annotated_ins);
                }
            }
        }
    }

    dict
}

fn get_loc_range(comment: Option<&str>) -> Option<(i32, i32)> {
    let comment = comment?;
    let pattern = r":(\d+):\d+\s*:\s*(\d+):\d+";
    let re = Regex::new(pattern).ok()?;
    let caps = re.captures(comment)?;

    let start = caps.get(1)?.as_str().parse::<i32>().ok()?;
    let end = caps.get(2)?.as_str().parse::<i32>().ok()?;
    if start != end {
        // Avoid repeated lines
        return None;
    }

    Some((start, end))
}
