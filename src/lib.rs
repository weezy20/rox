#![allow(unused)]

use std::{
    env::var_os,
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
    pub invalid: Vec<String>,
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
                    } else if c == ';' {
                        Self::set_bool(&mut rox, &line, false);
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
        // for some reason, Iterator::count() consumes the iter so we have this ugly code
        let word_count = line.clone().split_whitespace().count();
        let mut words = line.split_whitespace();
        // check for boolean
        if word_count == 1 {
            let word = words.next().unwrap().clone();
            if var_word!(word) {
                self.set_bool(word.clone(), true)
            } else {
                self.invalid.push(line.into());
            }
            return;
        }

        // Validate that the first two words don't contain invalid chars
        // if let Some(ident) = words.next() {
        //     if ident.chars().all(|ch| var_ch!(ch)) {
        //         if let Some(value) = words.next() {
        //             for &i_char in INVALID_TOKENS {
        //                 if value.contains(i_char) {
        //                     println!(
        //                         "Found a possible invalid \"value\" at line\n-> {line}",
        //                         line = line
        //                     );
        //                 }
        //             }
        //             self.switches.push((ident.to_string(), value.to_string()));
        //         }
        //     }
        // }
        let item: Option<(String, String)> = words
            .next()
            // Get and validate var name
            .and_then(|variable| {
                if var_word!(variable) {
                    Some(variable.to_string())
                } else {
                    None
                }
            })
            // Collect values
            .and_then(|variable| {
                let mut value = String::new();
                // for word in words {
                //     value.push_str(word);
                // }
                let index = line[variable.len()..]
                    .chars()
                    .position(|ch| !ch.is_ascii_whitespace())
                    .unwrap_or(variable.len());

                value.push_str(&line[variable.len()+index..]);
                Some((variable, value))
            });
        if let Some(item) = item {
            self.switches.push(item)
        } else {
            self.invalid.push(line.into());
        }
    }

    pub fn set_bool(&mut self, line: &str, boolean: bool) {
        let mut words = line.split_whitespace();
        if let Some(var) =
            words.find_map(|word| if var_word!(word) { Some(word) } else { None })
        {
            self.switches.push((var.into(), boolean.to_string()));
        } else {
            self.invalid.push(line.into());
        }
    }
}
#[macro_export]
macro_rules! var_word {
    ($word:expr) => {{
        let mut boolean = $word.chars().all(|ch| matches!(ch, 'A'..='Z' | '_'));
        boolean
    }};
}
#[macro_export]
macro_rules! var_ch {
    ($ch:expr) => {{
        matches!($ch, 'A'..='Z' | '_')
    }};
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
        let invalid = Vec::new();
        let mut comment_prefix = S!["#", r#"/*"#, r#"*/"#];
        Self {
            switches,
            comments,
            comment_prefix,
            invalid,
        }
    }
}
