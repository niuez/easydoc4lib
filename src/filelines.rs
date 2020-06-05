use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use regex::{ Regex, Captures };

#[derive(Debug, Clone)]
struct CommentBlock {
    start: usize,
    end: usize,
}

pub struct FileLines {
    lines: Vec<String>,
    code_lines: Vec<Option<usize>>,
    blocks: Vec<CommentBlock>,
}

impl FileLines {
    pub fn read(file: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut lines = Vec::new();
        for result in BufReader::new(File::open(file)?).lines() {
            let l = result?;
            lines.push(l);
        }
        let mut blocks = Vec::new();
        let mut code_lines = Vec::new();
        let mut start = None;
        let start_re = Regex::new(r"/\*\*")?;
        let end_re = Regex::new(r"\*\*/")?;
        for i in 0..lines.len() {
            start = match start {
                None => {
                    if start_re.captures(&lines[i]).is_some() {
                        Some(i)
                    }
                    else {
                        code_lines.push(Some(i));
                        None
                    }
                }
                Some(s) => {
                    if end_re.captures(&lines[i]).is_some() {
                        blocks.push( CommentBlock { start: s, end: i + 1 });
                        code_lines.push(None);
                        None
                    }
                    else {
                        Some(s)
                    }
                }
            }
        }
        Ok( Self { lines, blocks, code_lines })
    }

    pub fn get_comments(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut comments = Vec::new();

        let line_re = Regex::new(r"\* (.*$)")?;
        let white_line_re = Regex::new(r"\*")?;
        let quote_code_re = Regex::new(r"@(-?)(\d+)@")?;
        let remove_leading_ws_re = Regex::new(r"[\s]*(.*$)")?;
        for &CommentBlock { start, end } in self.blocks.iter() {
            for i in (start + 1)..(end - 1) {
                if let Some(cap) = line_re.captures(&self.lines[i]) {
                    let com = cap.get(1).unwrap().as_str().to_string();
                    let com = quote_code_re.replace_all(&com, |cap: &Captures| {
                        let minus = cap.get(1).unwrap().as_str().to_string();
                        let num = match cap.get(2).unwrap().as_str().parse::<usize>() {
                            Ok(n) => n,
                            Err(e) => return format!("{:?}", e),
                        };
                        let code = if minus == "-" {
                            self.lines[start - num].clone()
                        }
                        else {
                            self.lines[end + num].clone()
                        };
                        let code_cap = remove_leading_ws_re.captures(&code).unwrap();
                        code_cap.get(1).unwrap().as_str().to_string()
                    }).to_string();
                    comments.push(com);
                }
                else if let Some(_) = white_line_re.captures(&self.lines[i]) {
                    comments.push("".to_string());
                }
            }
        }
        Ok(comments)
    }

    pub fn get_code_lines(&self) -> Vec<String> {
        let mut codes = Vec::new();
        for i in self.code_lines.iter() {
            let s = match i {
                Some(i) => self.lines[*i].clone(),
                None => "".to_string(),
            };
            codes.push(s);
        }
        codes
    }
}
