use std::{
    collections::VecDeque,
    fs::File,
    io::prelude::*,
    process,
    str::FromStr,
};

#[derive(Default, Debug)]
pub struct Window {
    size: usize,
    inner: VecDeque<i32>,
}
impl Window {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            inner: VecDeque::new()
        }
    }
    pub fn push(&mut self, value: i32) {
        if self.inner.len() < self.size {
            self.inner.push_front(value);
        } else {
            self.inner.pop_back();
            self.inner.push_front(value);
        }
    }
    pub fn avg<'a>(&self) -> Option<i32> {
        if self.inner.len() != self.size {
            None
        } else {
            let sum: i32 = self.inner.iter().sum();
            Some(sum / self.size as i32)
        }
    }
}

pub fn moving_average(data: Vec<i32>, size: usize) -> i32 {
    let mut avgs: Vec<i32> = Vec::new();
    let mut window = Window::new(size);
    
    for v in data {
        window.push(v);
        let win_avg = window.avg();
        match window.avg() {
            Some(a) => {
                avgs.push(a);
            }
            None => {}
        }
        println!("window: {window:?} avg: {win_avg:?}");
    }
    todo!()
}

pub fn load_input() -> Vec<i32> {
    let file_path = "input";
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
    fn it_works() {
    }
}
