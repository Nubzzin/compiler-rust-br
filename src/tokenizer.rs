#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Principal,
    Imprimir,
    Sair,
    IntLit,
    StrLit,
    CurOpen,
    CurClose,
    ParenOpen,
    ParenClose,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub _type: TokenType,
    pub _value: Option<String>,
}

pub struct Tokenizer {
    src: String,
    index: usize,
}

impl Tokenizer {
    pub fn new(src: String) -> Self {
        Self { src, index: 0 }
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

                if buf == "principal" {
                    tokens.push(Token {
                        _type: TokenType::Principal,
                        _value: None,
                    });
                    buf.clear();
                    continue;
                }

                if buf == "imprimir" {
                    tokens.push(Token {
                        _type: TokenType::Imprimir,
                        _value: None,
                    });
                    buf.clear();
                    continue;
                }

                if buf == "sair" {
                    tokens.push(Token {
                        _type: TokenType::Sair,
                        _value: None,
                    });
                    buf.clear();
                    continue;
                }
                tokens.push(Token {
                    _type: TokenType::StrLit,
                    _value: Some(buf.clone()),
                });
                buf.clear();
                continue;
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

            if self.peek(0).unwrap() == '{' {
                tokens.push(Token {
                    _type: TokenType::CurOpen,
                    _value: None,
                });
                buf.clear();
                self.consume();
                continue;
            }
            if self.peek(0).unwrap() == '}' {
                tokens.push(Token {
                    _type: TokenType::CurClose,
                    _value: None,
                });
                buf.clear();
                self.consume();
                continue;
            }
            if self.peek(0).unwrap() == '(' {
                tokens.push(Token {
                    _type: TokenType::ParenOpen,
                    _value: None,
                })
            }
            if self.peek(0).unwrap() == ')' {
                tokens.push(Token {
                    _type: TokenType::ParenClose,
                    _value: None,
                })
            }
            if self.peek(0).unwrap().is_whitespace() {
                buf.clear();
                self.consume();
                continue;
            }
            self.consume();
        }
        tokens
    }
}
