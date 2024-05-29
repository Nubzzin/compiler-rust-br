section .data
    text db "matheus", 10

section .text
    global _start
_start:
    mov rax, 1
    mov rdi, 1
    mov rsi, text
    mov rdx, 8
    syscall
    mov rax, 60
    mov rdi, 69
    syscall
