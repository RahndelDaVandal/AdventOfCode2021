use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::path;

const MAX_LEN: usize = 80;

#[derive(Debug)]
pub struct Config {
    pub cwd: String,
    pub path: String,
}
impl Config {
    /// Build cfg from CLI args
    pub fn build(
        mut args: impl Iterator<Item = String> + std::fmt::Debug,
    ) -> Result<Config, &'static str> {
        log::debug!("env::args: {:?}", args);
        let cwd = args.next().unwrap();
        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        Ok(Config { cwd, path })
    }
}

pub fn format_md_file(cfg: Config) {
    log::debug!("format_md_file: path=\"{}\"", cfg.path);
    let file_string = file_to_string(&cfg.path);
    log::debug!("format_md_file: file_string=\n\"{:?}\"", file_string);
    let formatted_file_string = fmt_file_string(file_string);
    string_to_file(cfg, formatted_file_string);
}

/// Read file from path to String
fn file_to_string(path: &String) -> String {
    match File::open(path) {
        Ok(mut f) => {
            let mut contents: FileString = String::new();
            match f.read_to_string(&mut contents) {
                Ok(_) => contents,
                Err(e) => {
                    eprintln!("Error reading {} to string: {}", path, e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error opening {}: {}", path, e);
            process::exit(1);
        }
    }
}

pub trait StringToLines {
    fn to_lines(&self) -> Vec<String>;
}
pub type FileString = String;
pub type Lines = Vec<String>;
impl StringToLines for FileString {
    fn to_lines(&self) -> Vec<String> {
        let mut lines: Lines = Vec::new();
        let mut curr_line = String::new();
        let chars: Vec<char> = self.chars().collect();
        for (i, c) in chars.iter().enumerate() {
            if c == &'\n' {
                curr_line.push(*c);
                if i + 1 < chars.len() - 1 && chars[i + 1] != '\n' {
                    lines.push(curr_line.to_string());
                    curr_line.clear();
                }
            } else {
                curr_line.push(*c);
            }
        }
        lines
    }
}

/// Format file_string
fn fmt_file_string(file_string: String) -> String {
    let mut output = String::new();
    let lines: Vec<String> = file_string.to_lines();
    log::debug!("num of lines: {}", lines.len());
    for l in lines {
        output.push_str(&split(l));
    }
    output
}

fn split(line: String) -> String {
    if line.len() <= MAX_LEN {
        return line;
    }

    let chars: Vec<char> = line.chars().collect();

    let mut idx = MAX_LEN - 1;
    loop {
        idx -= 1;
        let c = chars[idx];
        if c == ' ' {
            break;
        }
        if c == '\n' {
            panic!("Tried to split at \'\n\'");
        }
    }

    let mut new_line: String = chars[0..idx].iter().cloned().collect::<String>();
    new_line.push('\n');
    let mut output = String::from(new_line.as_str());

    let remaining: String = chars[idx + 1..].iter().cloned().collect::<String>();

    output.push_str(split(remaining).as_str());
    output
}

/// Save file_string to file
fn string_to_file(cfg: Config, file_string: String) {
    // let file_name: Vec<&str> = cfg.path.split('.').collect();
    // let stem = file_name[0];
    let file_path = path::Path::new(&cfg.path).canonicalize().unwrap();
    log::debug!("file_path: {:?}", file_path);

    // let new_name = format!(
    //     "{}/{}_fmtd.md",
    //     file_path
    //         .parent()
    //         .unwrap()
    //         .parent()
    //         .unwrap()
    //         .display(),
    //     stem
    // );
    // log::debug!("string_to_file::new_name: {}", new_name);

    match File::create(file_path) {
        Ok(mut f) => {
            match write!(f, "{}", file_string) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error writing file_string to file: {e}");
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error creatting new file: {e}");
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn passes() {
        // assert_eq!(result, true);
    }
    #[test]
    fn fails() {
        // assert_eq!(result, false);
    }
}
