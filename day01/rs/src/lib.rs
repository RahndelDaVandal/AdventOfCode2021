use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::str::FromStr;

pub fn count_increase(data: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut last:Option<i32> = None;
    for v in data {
        match last {
            Some(x) => {
                if v > x {
                    count += 1;
                }
            },
            None => {},
        }
        last = Some(v);
    }
    count
}

pub fn load_input() -> Vec<i32> {
    let file_path = "../input";
    match File::open(file_path) {
        Ok(mut f) => {
            let mut contents = String::new();
            match f.read_to_string(&mut contents) {
                Ok(_) => {
                    let mut split: Vec<&str> = contents.split('\n').collect();
                    if split[split.len() - 1].is_empty() {
                        split.pop();
                    }
                    let o: Vec<i32> = split
                        .iter()
                        .map(|x| FromStr::from_str(x).unwrap())
                        .collect();
                    o
                }
                Err(e) => {
                    eprintln!("Error reading file to string: {e}");
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error opening input file: {e}");
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn it_works() {}
}
