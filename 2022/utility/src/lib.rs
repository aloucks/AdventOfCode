use std::env;
use std::io::prelude::BufRead;
use std::{fs::File, io::BufReader};

// hacky, but who cares
pub use anyhow::anyhow as format_err;
pub use anyhow::Error;
pub use anyhow::Result;

pub fn get_cargo_manifest_dir() -> Result<String> {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    Ok(cargo_manifest_dir)
}

pub fn get_file_as_vec_string(filename: &str) -> Result<Vec<String>> {
    let reader = BufReader::new(File::open(filename)?)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    Ok(reader)
}

pub fn get_file_delimiters(input: &str) -> (&'static str, &'static str) {
    let group_delim = if input.contains("\r\n\r\n") {
        "\r\n\r\n"
    } else {
        "\n\n"
    };

    let line_delim = if input.contains("\r\n") { "\r\n" } else { "\n" };

    (line_delim, group_delim)
}

pub fn get_input_path() -> Result<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(format_err!(
            "need to pass input text path as first argument"
        ));
    }

    Ok(args[1].clone())
}
