section .data
    text0 db "Olá ", 10
    text1 db "Matheus ", 10
    text2 db "Como você vai ", 10

section .text
    global _start
_start:
    mov rax, 1
    mov rdi, 1
    mov rsi, text0
    mov rdx, 6
    syscall
    mov rax, 1
    mov rdi, 1
    mov rsi, text1
    mov rdx, 9
    syscall
    mov rax, 1
    mov rdi, 1
    mov rsi, text2
    mov rdx, 16
    syscall
    mov rax, 60
    mov rdi, 69
    syscall
