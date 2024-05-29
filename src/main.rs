pub mod constructor;
pub mod parser;
pub mod tokenizer;

use constructor::Construct;
use core::panic;
use parser::Parser;
use std::env;
use std::fs;
use std::process;
use tokenizer::Token;
use tokenizer::Tokenizer;

fn main() {
    let contents;
    {
        let args: Vec<String> = env::args().collect();
        if args.len() == 2 {
            let file = fs::read_to_string(&args[1]).unwrap_or_else(|err| {
                println!("Erro na leitura do arquivo -> \"{err}\"");
                process::exit(1)
            });
            contents = file;
        } else {
            println!("Argumentos incorretos!");
            process::exit(1)
        }
    }

    let mut tokenizer = Tokenizer::new(contents);
    let tokens: Vec<Token> = tokenizer.tokenize();

    // DEBUG
    // println!("TOKENS VVVVV");
    // println!("{tokens:#?}");
    // \DEBUG

    let mut parser = Parser::new(tokens);
    let ast = parser.parser();

    // DEBUG
    // println!("AST VVVVV");
    // println!("{ast:#?}");
    // \DEBUG

    let mut constructor = Construct::new(ast);
    constructor.gen_asm();

    let nasm_command = process::Command::new("nasm")
        .arg("-f")
        .arg("elf64")
        .arg("-o")
        .arg("a.o")
        .arg("a.asm")
        .output();
    if !nasm_command.unwrap().status.success() {
        panic!("Falha ao executar nasm");
    }
    let ld_command = process::Command::new("ld")
        .arg("-o")
        .arg("output")
        .arg("a.o")
        .output();
    if !ld_command.unwrap().status.success() {
        panic!("Falha ao executar ld");
    }
    let _rm_ld = process::Command::new("rm").arg("a.o").output();
    // let _rm_asm = process::Command::new("rm").arg("a.asm").output(); // Debug

    println!("Sucesso!");
}
