// cargo-deps: regex="1.3.1"
extern crate regex;

use regex::Regex;
use std::error::Error;
use std::fs::write;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn create_readme() -> Result<(), Box<dyn Error>> {
    let comment_capture = Regex::new(r"^//!\s?(?P<comment>.*)")?;
    // Get the contents of the README from the top of lib.rs
    let src_lib = File::open("src/bin/vocab.rs").expect("Could not read lib.rs");
    let reader = BufReader::new(src_lib);

    let mut readme_lines = vec![];
    for result in reader.lines() {
        let line = result.expect("Could not read line");
        if let Some(capture) = comment_capture.captures_iter(&line).next() {
            if let Some(comment) = capture.name("comment") {
                readme_lines.push(comment.as_str().to_string())
            }
        }
    }
    let mut contents: String = readme_lines.join("\n");
    contents.push_str("\n");
    write("README.md", contents).expect("Could not write to README.md");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    create_readme()?;
    Ok(())
}
