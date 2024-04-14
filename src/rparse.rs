use std::error::Error;
use std::fmt::{Debug, Display, Formatter, write};
use std::fs::{File};
use std::io::Read;
use regex::Regex;

static COMMA: char = '\x2C';
static LF: char = '\x0A';

static CR: char = '\x0D';

#[derive(Debug)]
struct FileAccessError;

impl Error for FileAccessError {}

impl Display for FileAccessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot access file. Are you sure this file exists?")
    }
}

pub struct CSVParser<'a> {
    file: File,
    has_headers: bool,
    delimiter: &'a str,
}

impl <'a> CSVParser<'a> {
    pub fn new(file_name: &str, has_headers: bool, delimiter: &'a str) -> Result<CSVParser<'a>, FileAccessError> {
        if let Ok(f) = File::open(file_name) {
            return Ok(CSVParser { file: f, has_headers, delimiter });
        }
        Err(FileAccessError)
    }

    pub fn each_line<F, T>(&mut self, func: F) where F: Fn() -> T {
        let all_lines = self.read_all();
    }


    fn read_all(&mut self) -> Vec<Vec<String>> {
        let mut buf = String::new();
        let field_regex = Regex::new(format!(r#"(("\w| |)[\w\d]+("|)(?<![{}\n\r]))"#, self.delimiter).as_str()).unwrap(); // Quotes and commas if available

        self.file.read_to_string(&mut buf).expect("EOF not detected in file");

        buf
            .split(|ch| ch == CR || ch == LF)
            .map(|line| line.to_string())
            .map(|mut line| {
                line.push(LF);
                line
            })
            .map(|mut line| {
                field_regex
                    .find_iter(line.as_str())
                    .map(|found| found.as_str()
                        .trim()
                        .trim_end_matches('"')
                        .trim_start_matches('"')
                        .trim().to_string()
                    )
                    .collect::<Vec<String>>()
            }).collect()
    }
}
