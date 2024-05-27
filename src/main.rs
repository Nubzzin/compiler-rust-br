pub mod parser;
pub mod tokenizer;

use parser::Parser;
use std::env;
use std::fs;
use tokenizer::Token;
use tokenizer::Tokenizer;

fn main() {
    let file: String;

    {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            file = fs::read_to_string(&args[1]).expect("Falha ao ler arquivo");
        } else {
            panic!("Argumentos incorretos!");
        }
    }

    let mut tokenizer = Tokenizer::new(file);
    let tokens: Vec<Token> = tokenizer.tokenize();

    println!("TOKENS VVVVV");
    println!("{tokens:#?}");

    let mut parser = Parser::new(tokens);
    let ast = parser.parser();

    println!("AST VVVVV");
    println!("{ast:#?}");
}
