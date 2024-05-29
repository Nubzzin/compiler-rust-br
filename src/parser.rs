use crate::tokenizer::{Token, TokenType};
use core::panic;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Imprimir(NodeImprimir),
    Sair(NodeSair),
}

#[derive(Debug, PartialEq)]
pub struct NodeSairExpr {
    pub _value_expr: Token,
}

#[derive(Debug, PartialEq)]
pub struct NodeSair {
    pub _value: NodeSairExpr,
}

#[derive(Debug, PartialEq)]
pub struct NodeImprimirExpr {
    pub _value_expr: Token,
}

#[derive(Debug, PartialEq)]
pub struct NodeImprimir {
    pub _value: NodeImprimirExpr,
}

#[derive(Debug)]
pub struct NodePrincipalExpr {
    pub _value_expr: Expr,
}

#[derive(Debug)]
pub struct NodePrincipal {
    pub values: Vec<NodePrincipalExpr>,
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    fn peek(&self, offset: isize) -> Option<Token> {
        if self.tokens.len() > self.index && (self.index) as isize - offset >= 0 {
            Some(self.tokens[self.index - offset as usize].clone())
        } else {
            None
        }
    }

    fn consume(&mut self) -> Option<Token> {
        if self.tokens.len() > self.index {
            let tmp = self.index;
            self.index += 1;
            Some(self.tokens[tmp].clone())
        } else {
            None
        }
    }

    pub fn parser(&mut self) -> NodePrincipal {
        if self.tokens.len() == 0 {
            panic!("Sem argumentos para inicialização!");
        }
        let mut node_principal: NodePrincipal;

        if self.peek(0).unwrap()._type == TokenType::Principal {
            self.consume();
            node_principal = NodePrincipal { values: vec![] };
            if self.peek(0).unwrap()._type == TokenType::ParenOpen {
                self.consume();
                if self.peek(0).unwrap()._type == TokenType::ParenClose {
                    self.consume();
                } else {
                    panic!("Falha ao fechar parenteses de \"principal\"");
                }
            } else {
                panic!("Falha ao abrir parenteses de \"principal\"");
            }
        } else {
            panic!("Argumento \"principal\" necessário");
        }
        if self.peek(0).unwrap()._type == TokenType::CurOpen {
            self.consume();
            if self.tokens[self.tokens.len() - 1]._type != TokenType::CurClose {
                panic!("Falha ao fechar curly de \"principal\"");
            }
        } else {
            panic!("Falha ao abrir curly de \"principal\"");
        }
        while self.peek(0).is_some() {
            if self.peek(0).unwrap()._type == TokenType::Imprimir {
                let node_imprimir: NodeImprimir;
                self.consume();
                if self.peek(0).unwrap()._type == TokenType::ParenOpen {
                    self.consume();
                    if self.peek(0).unwrap()._type == TokenType::IntLit {
                        let node_imprimir_expr = NodeImprimirExpr {
                            _value_expr: self.peek(0).unwrap(),
                        };
                        node_imprimir = NodeImprimir {
                            _value: node_imprimir_expr,
                        };
                        self.consume();
                    } else if self.peek(0).unwrap()._type == TokenType::Aspas {
                        self.consume();
                        if self.peek(0).unwrap()._type == TokenType::StrLit {
                            let node_imprimir_expr = NodeImprimirExpr {
                                _value_expr: self.peek(0).unwrap(),
                            };
                            node_imprimir = NodeImprimir {
                                _value: node_imprimir_expr,
                            };
                            self.consume();
                        } else {
                            panic!("Argumentos de \"imprimir\" inválidos!");
                        }
                        if self.peek(0).unwrap()._type != TokenType::Aspas {
                            panic!("Argumentos de \"imprimir\" inválidos!");
                        }
                        self.consume();
                    } else {
                        panic!("Argumentos de \"imprimir\" inválidos!");
                    }
                } else {
                    panic!("Falha ao abrir parenteses de \"imprimir\"");
                }
                if self.peek(0).unwrap()._type == TokenType::ParenClose {
                    self.consume();
                } else {
                    panic!("Falha ao fechar parenteses de \"imprimir\"");
                }
                let node_principal_expr = NodePrincipalExpr {
                    _value_expr: Expr::Imprimir(node_imprimir),
                };
                node_principal.values.push(node_principal_expr);
                continue;
            }
            if self.peek(0).unwrap()._type == TokenType::Sair {
                let node_sair: NodeSair;
                self.consume();
                if self.peek(0).unwrap()._type == TokenType::ParenOpen {
                    self.consume();
                    if self.peek(0).unwrap()._type == TokenType::IntLit {
                        let node_sair_expr = NodeSairExpr {
                            _value_expr: self.peek(0).unwrap(),
                        };
                        node_sair = NodeSair {
                            _value: node_sair_expr,
                        };
                        self.consume();
                    } else {
                        panic!("Argumentos de \"sair\" inválidos!");
                    }
                } else {
                    panic!("Falha ao abrir parenteses de \"sair\"");
                }
                if self.peek(0).unwrap()._type == TokenType::ParenClose {
                    self.consume();
                } else {
                    panic!("Falha ao fechar parenteses de \"sair\"");
                }
                let node_principal_expr = NodePrincipalExpr {
                    _value_expr: Expr::Sair(node_sair),
                };
                node_principal.values.push(node_principal_expr);
                continue;
            }
            if self.peek(0).unwrap()._type == TokenType::StrLit
                || self.peek(0).unwrap()._type == TokenType::IntLit
            {
                panic!(
                    "Argumento incorreto dentro de \"principal\" -> \"{}\" não reconhecido",
                    self.peek(0).unwrap()._value.unwrap()
                );
            }
            self.consume();
        }
        node_principal
    }
}
