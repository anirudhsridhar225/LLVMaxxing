mod ast;
mod codegen;
mod isa;
mod lexer;
mod parser;
mod type_checker;
mod utils;

//use inkwell::context;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input.c>", args[0]);
        std::process::exit(1);
    }

    let input_path = Path::new(&args[1]);
    let source_code = fs::read_to_string(input_path).expect("Failed to read source file");

    //Lexical analysis
    let tokens = lexer::tokenize(&source_code);
    println!("Tokens: {:?}", tokens);
}
