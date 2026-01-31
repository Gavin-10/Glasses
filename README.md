#Welcome to Glasses
###*You need Glasses to C*
---
##Dependencies
-Linux OS
-Rust
-GCC added to path

##Usage
`Glasses <filename> (--lex | --parse | --codegen)?`

This is my hand written C compiler, written in Rust. It's job
is to take a written C program, optimize it, and translate it,
to x64 assembly for Linux. After that, it hands the assembly off
to GCC to complete the assembler and linking phases. Feel free
to download it, build it, and give it a shot!.
