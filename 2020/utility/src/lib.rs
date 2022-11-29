use std::io::prelude::BufRead;
use std::{fs::File, io::BufReader};

// hacky, but who cares
pub use anyhow::anyhow as format_err;
pub use anyhow::Error;
pub use anyhow::Result;

pub fn get_file_as_vec_string(filename: &str) -> Result<Vec<String>> {
    let reader = BufReader::new(File::open(filename)?)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    Ok(reader)
}
