pub mod parser;
pub mod tokenizer;

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

    println!("{:?}", tokens);
}
