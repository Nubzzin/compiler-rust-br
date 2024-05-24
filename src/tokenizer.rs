#[derive(Debug)]
enum TokenType {
    Main,
    Exit,
    IntLit,
}

#[derive(Debug)]
pub struct Token {
    _type: TokenType,
    _value: Option<String>,
}

pub struct Tokenizer {
    src: String,
    index: usize,
}

impl Tokenizer {
    pub fn new(src: String) -> Tokenizer {
        Tokenizer { src, index: 0 }
    }

    fn peek(&self, offset: isize) -> Option<char> {
        if self.src.chars().nth(self.index) != None && (self.index as isize) - offset >= 0 {
            self.src
                .chars()
                .nth(((self.index as isize) + offset) as usize)
        } else {
            None
        }
    }

    fn consume(&mut self) -> Option<char> {
        if self.src.chars().nth(self.index).is_some() {
            let tmp = self.index;
            self.index += 1;
            self.src.chars().nth(tmp)
        } else {
            None
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut buf = String::new();

        while self.peek(0).is_some() {
            if self.peek(0).unwrap().is_alphabetic() {
                buf.push(self.peek(0).unwrap());
                self.consume();
                while self.peek(0).unwrap().is_alphanumeric() {
                    buf.push(self.peek(0).unwrap());
                    self.consume();
                }

                if buf == "main" {
                    tokens.push(Token {
                        _type: TokenType::Main,
                        _value: None,
                    });
                    buf.clear();
                    continue;
                }

                if buf == "exit" {
                    tokens.push(Token {
                        _type: TokenType::Exit,
                        _value: None,
                    });
                    buf.clear();
                    continue;
                }
            }

            if self.peek(0).unwrap().is_numeric() {
                buf.push(self.peek(0).unwrap());
                self.consume();
                while self.peek(0).unwrap().is_numeric() {
                    buf.push(self.peek(0).unwrap());
                    self.consume();
                }
                tokens.push(Token {
                    _type: TokenType::IntLit,
                    _value: Some(buf.clone()),
                });
                buf.clear();
                continue;
            }
            self.consume();
        }
        tokens
    }
}
