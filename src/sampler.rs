use anyhow::Result;
use polars::prelude::*;

pub fn sample_gen(df: &DataFrame) -> Result<(String, String)> {
    // Unwrap struct and check for status == "OK"
    let result = df
        .clone()
        .lazy()
        .select([col("name"), col("results")])
        .explode(["results"])
        .unnest(["results"])
        .filter(col("status").eq(lit("OK")))
        .limit(1)
        .collect()?;

    // Sample a single correct gen (Status == "OK")
    let name = result
        .column("name")?
        .get(0)?
        .get_str()
        .unwrap()
        .to_string();
    let program = result
        .column("program")?
        .get(0)?
        .get_str()
        .unwrap()
        .to_string();

    // These to_string() do not escape the strings, as we get returned "Foo" instead of a
    // string Foo
    Ok((name, program))
}
