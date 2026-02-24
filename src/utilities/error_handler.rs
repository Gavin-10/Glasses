
use std::process;

use crate::utilities::file_cleanup::*;

pub fn emission_error(msg: &str) -> ! {
    println!("Emission Error:\n{}", msg);

    process::exit(1);
}

pub fn parser_error_no_line(msg: &str) -> ! {
    println!("Parser Error:\n{}", msg);

    process::exit(1);
}

pub fn parser_error(line: u32, msg: &str) -> ! {
    println!("Parser Error:\nLine {}: {}", line, msg);

    process::exit(1);
}

pub fn resolver_error(msg: &str) -> ! {
    println!("Resolver Error:\n{}", msg);

    process::exit(1);
}

pub fn fmt_lexer_error(line: u32, msg: &str) -> String {
    format!("Lexer Error:\nLine {}: {}", line, msg)
}

pub fn error_and_clean(msg: &str, file: &str) -> ! {
    clean_file(file);
    println!("{}", msg);

    process::exit(1);
}