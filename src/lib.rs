#![allow(unused)]

use std::{fs::File, io::{BufRead, BufReader}};
/// Attempt to parse the file as a `.rox` file whose
/// specification matches the test.rox file
pub struct Rox {
    /// A vector of identifier-value pairs
    switches: Vec<(String, String)>,
    comments: Vec<String>,
    comment_prefix: Vec<String>,
}
impl Rox {
    pub fn interpret(file_path: &str) -> std::io::Result<()> {
        let reader = BufReader::new(File::open(file_path)?);
        for (n , line) in reader.lines().enumerate() {
            println!("Line number {n:0>width$} -> {line}", n=n, line=line?, width=2);
        }
        let mut rox = Self::default();
        Ok(())
    }
}

impl std::default::Default for Rox {
    fn default() -> Self {
        let switches = Vec::new();
        let comments = Vec::new();
        let mut comment_prefix = Vec::new();
        // treat `;` and `#` as comment prefixes
        comment_prefix.push("#".to_string());
        comment_prefix.push(";".to_string());
        Self {
            switches,
            comments,
            comment_prefix,
        }
    }
}
