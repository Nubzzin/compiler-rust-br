use crate::parser::{Expr, NodeImprimir, NodeImprimirExpr, NodePrincipal, NodeSair};
use crate::tokenizer::{Token, TokenType};
use std::fs::File;
use std::io::prelude::*;

pub struct Construct {
    pub ast: NodePrincipal,
    pub index: usize,
    bufbody: String,
    bufdata: String,
}

impl Construct {
    pub fn new(ast: NodePrincipal) -> Self {
        Self {
            ast,
            index: 0,
            bufbody: String::new(),
            bufdata: String::new(),
        }
    }

    pub fn gen_asm(&mut self) {
        let mut file = File::create("a.asm").unwrap();
        let mut iter = self.ast.values.iter();

        self.bufdata.push_str("section .data\n");
        self.bufbody
            .push_str("section .text\n    global _start\n_start:\n");

        for value in iter.clone() {
            match &value._value_expr {
                Expr::Imprimir(node_imprimir) => {
                    if node_imprimir._value._value_expr._type == TokenType::IntLit {
                        self.bufdata.push_str("    text db ");
                        self.bufdata
                            .push_str(node_imprimir._value._value_expr._value.as_ref().unwrap());
                        self.bufdata.push_str(", 10\n");
                    } else {
                        self.bufdata.push_str("    text db \"");
                        self.bufdata
                            .push_str(node_imprimir._value._value_expr._value.as_ref().unwrap());
                        self.bufdata.push_str("\", 10\n");
                    }

                    self.bufbody.push_str("    mov rax, 1\n");
                    self.bufbody.push_str("    mov rdi, 1\n");
                    self.bufbody.push_str("    mov rsi, text\n");
                    self.bufbody.push_str("    mov rdx, ");
                    let size = &node_imprimir
                        ._value
                        ._value_expr
                        ._value
                        .as_ref()
                        .unwrap()
                        .len()
                        + 1;
                    self.bufbody.push_str(size.to_string().as_ref());
                    self.bufbody.push_str("\n    syscall\n");
                }
                Expr::Sair(node_sair) => {
                    self.bufbody.push_str("    mov rax, 60\n");
                    self.bufbody.push_str("    mov rdi, ");
                    self.bufbody
                        .push_str(&node_sair._value._value_expr._value.as_ref().unwrap());
                    self.bufbody.push_str("\n    syscall\n");
                }
            }
        }

        file.write_all(&self.bufdata.as_bytes());
        file.write_all("\n".as_bytes());
        file.write_all(&self.bufbody.as_bytes());
    }
}
