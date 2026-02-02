
use std::env;
use std::process;
use std::process::Command;
use std::fs;

mod lexer;
mod utilities;
mod parser;
mod tacky;
mod code_gen;
mod code_emission;

use lexer::lexer_ops::lex;
use utilities::file_cleanup::*;
use utilities::error_handler::*;
use parser::tree_builder::parse;
use parser::ast_printer::print_ast;
use tacky::t_tree_builder::gen_tacky;
use code_gen::a_tree_builder::gen_code;
use code_emission::write_assembly::output;

fn args_error()  -> ! {
    println!("Usage: glasses <filename> --lex? | --parse? | --codegen?");

    process::exit(1);
}

fn check_args(args: &Vec<String>) -> Option<&str> {
    if args.len() < 2 || args.len() > 3 {
        args_error();
    } else if args.len() == 3 {
        match args[2].as_str() {
            "--lex" => Some("--lex"),
            "--parse" => Some("--parse"),
            "--codegen" => Some("--codegen"),
            "--tacky" => Some("--tacky"),
            _ => args_error(),
        }
    } else {
        None
    }
}

fn preprocess(name: &str) {
    let output = Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(format!("{}.c", name))
        .arg("-o")
        .arg(format!("{}.i", name))
        .output().unwrap_or_else(|e| panic!("Failed to inflect preprocessor: {}", e));

    if !output.status.success() {
        println!("Failed to execute preprocessor");
        process::exit(1);
    }
}

fn compile(name: &str, flag: Option<&str>) {

    let buffer: Vec<u8> = fs::read(&format!("{}.i", name)).unwrap();

    let tokens =  match lex(buffer) {
        Ok(res) => res,
        Err(msg) => error_and_clean(msg.as_str(), &format!("{}.i", name)),
    };
    clean_file(&format!("{}.i", name));
    if flag == Some("--lex") {
        println!("{:?}", tokens);
        process::exit(0);
    }

    let program_ast = parse(tokens);
    if flag == Some("--parse") {
        print_ast(program_ast);
        process::exit(0);
    }

    let tacky_ir = gen_tacky(program_ast);
    if flag == Some("--tacky") {
        println!("{:?}", tacky_ir);
        process::exit(0);
    }

    let assembly_tree = gen_code(tacky_ir);
    output(assembly_tree, name);
    if flag == Some("--codegen") {
        process::exit(0);
    }
}

fn assemble(name: &str) {
    let output = Command::new("gcc")
        .arg(format!("{}.s", name))
        .arg("-o")
        .arg(&name)
        .output().unwrap_or_else(|e| panic!("Failed to inflect assembler: {}", e));

    let _ = fs::remove_file(&format!("{}.s", name).to_string());

    if !output.status.success() {
        println!("Failed to execute assembler");
        process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();


    let flag = check_args(&args);
    let name = &args[1][..&args[1].len() - 2];

    preprocess(&name);
    compile(&name, flag);
    assemble(&name);
}
