#![allow(unused)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};
/// Attempt to parse the file as a `.rox` file whose
/// specification matches the test.rox file
pub struct Rox {
    /// A vector of identifier-value pairs
    pub switches: Vec<(String, String)>,
    pub comments: Vec<String>,
    comment_prefix: Vec<String>,
}
impl Rox {
    pub fn interpret(file_path: &str) -> std::io::Result<Self> {
        let mut rox = Self::default();
        let reader = BufReader::new(File::open(file_path)?);

        'outer: for (n, line) in reader.lines().enumerate() {
            // println!("Line number {n:0>width$} -> {line}", n=n, line=line?, width=2);
            let line = line?;
            // Check for comments or parse the line
            // comments begin with "#", "/*", and end with "*/" optionally
            // todo!("implement multi-line comments")
            // let mut comment: bool = false;

            let mut chars = line.chars();
            match chars.next() {
                Some(c) => {
                    if c == '#' {
                        rox.comments.push(line);
                    } else {
                        Self::switch_check(&mut rox, &line);
                    }
                }
                None => continue 'outer,
            }
        }
        Ok(rox)
    }
    /// given a line and a Rox instance, create a variable value pair
    /// Variables need to be uppercase only
    /// and not contain any chars mentioned in INVALID_TOKENS

    pub fn switch_check(&mut self, line: &str) {
        let mut words = line.split_whitespace();
        // Validate that the first two words don't contain invalid chars
        if let Some(ident) = words.next() {
            if ident.chars().all(|ch| matches!(ch, 'A'..='Z' | '_')) {
                if let Some(value) = words.next() {
                    for &i_char in INVALID_TOKENS {
                        if value.contains(i_char) {
                            println!(
                                "Found a possible invalid \"value\" at line\n-> {line}",
                                line = line
                            );
                        }
                    }
                    self.switches.push((ident.to_string(), value.to_string()));
                }
            }
        }
    }
}
static INVALID_TOKENS: &[char] = &[
    '-', '`', '?', '$', '%', '+', '/', '\\', '{', '}', '[', ']', '\'', '"', ',', '.',
    '<', '>', ':', ';', '#', '&', '^', '$', '*', '(', ')', '=',
];
impl std::default::Default for Rox {
    fn default() -> Self {
        macro_rules! S {
            // Create a vector of owned strings from static str
            // writing to_string() is too painful
            [$($str:expr),+] => {
                {
                    let mut vec = vec![];
                    $(
                        vec.push($str.to_string());
                    )+
                    vec
                }
            };
        }
        let switches = Vec::new();
        let comments = Vec::new();
        let mut comment_prefix = S!["#", r#"/*"#, r#"*/"#];
        Self {
            switches,
            comments,
            comment_prefix,
        }
    }
}
