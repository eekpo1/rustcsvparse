use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, write};
use std::fs::{File};
use std::io::Read;
use fancy_regex::Regex;

static COMMA: char = '\x2C';
static LF: char = '\x0A';

static CR: char = '\x0D';

#[derive(Debug)]
pub struct FileAccessError;

impl Error for FileAccessError {}

impl Display for FileAccessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot access file. Are you sure this file exists?")
    }
}

pub struct CSVParser<'a> {
    file: String,
    has_headers: bool,
    delimiter: &'a str,
}

impl <'a> CSVParser<'a> {
    pub fn new(file_name: &str, has_headers: bool, delimiter: &'a str) -> Result<CSVParser<'a>, FileAccessError> {
        if let Ok(mut f) = File::open(file_name) {
            let mut buf = String::new();
            f.read_to_string(&mut buf).expect("EOF not detected in file");
            return Ok(CSVParser { file: buf, has_headers, delimiter });
        }
        Err(FileAccessError)
    }

    pub fn each_line<F: Fn(String) -> String>(&mut self, func: F) -> Vec<String>  {
        let all_lines = self.read_all();

        all_lines
            .iter()
            .map(|line_vector| line_vector.join(","))
            .map(|line| func(line))
            .collect::<Vec<String>>()
    }

    pub fn with_headers(&mut self) -> Vec<BTreeMap<String, String>> {
        let mut all_lines = self.read_all().clone();
        let headers = all_lines.remove(0);
        all_lines
            .iter()
            .map(|line_vec| {
                line_vec
                    .into_iter()
                    .enumerate()
                    .map(|(i, item)| (headers[i].clone(), item.clone()))
                    .collect::<BTreeMap<String, String>>()
            }).collect::<Vec<BTreeMap<String, String>>>()
    }


    fn read_all(&self) -> Vec<Vec<String>> {
        let field_regex = Regex::new(format!(r#"(("\w| |)[\w\d]+("|)(?<![{}\n\r]))"#, self.delimiter).as_str()).unwrap(); // Quotes and commas if available

        self.file
            .clone()
            .split(|ch| ch == CR || ch == LF)
            .map(|line| line.to_string())
            .filter(|line| !line.is_empty())
            .map(|mut line| {
                line.push(LF);
                line
            })
            .map(|mut line| {
                field_regex
                    .find_iter(line.as_str())
                    .map(|found| found.expect("Malformed row detected.").as_str()
                        .trim()
                        .trim_end_matches('"')
                        .trim_start_matches('"')
                        .trim().to_string()
                    )
                    .collect::<Vec<String>>()
            }).collect()
    }
}
