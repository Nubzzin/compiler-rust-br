use crate::parser::{Expr, NodePrincipal};
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
        let iter = self.ast.values.iter();
        let mut counter = 0;

        self.bufdata.push_str("section .data\n");
        self.bufbody
            .push_str("section .text\n    global _start\n_start:\n");

        for value in iter.clone() {
            match &value._value_expr {
                Expr::Imprimir(node_imprimir) => {
                    self.bufdata.push_str("    text");
                    self.bufdata.push_str(&counter.to_string());
                    self.bufdata.push_str(" db \"");
                    self.bufdata
                        .push_str(node_imprimir._value._value_expr._value.as_ref().unwrap());
                    self.bufdata.push_str("\", 10\n");

                    self.bufbody.push_str("    mov rax, 1\n");
                    self.bufbody.push_str("    mov rdi, 1\n");
                    self.bufbody.push_str("    mov rsi, text");
                    self.bufbody.push_str(&counter.to_string());
                    self.bufbody.push_str("\n    mov rdx, ");
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
                    counter += 1;
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

        let _ = file.write_all(&self.bufdata.as_bytes());
        let _ = file.write_all("\n".as_bytes());
        let _ = file.write_all(&self.bufbody.as_bytes());
    }
}
