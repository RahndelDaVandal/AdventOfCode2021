#![allow(non_snake_case)]
use std::{env, process};
use fmtmd::{Config, format_md_file};

//  WorkFlow
//  1 - Get file_path from cli args
//  2 - Read in file to String
//  3 - Iterate over String
//      - Find char at MaxLineLength
//      - If in word go to last whitespace
//      - Change whitespace to '\n'
//  4 - Save to file

fn main() {
    env_logger::init();

    log::debug!("cwd: {:?}", env::current_dir());

    // Get path from cli args
    let cfg = Config::build(env::args()).unwrap_or_else(|err| {
        log::error!("Proplem parsing arguments: {err}");
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    format_md_file(cfg);
}
