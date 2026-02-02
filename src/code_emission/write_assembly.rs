
use crate::code_gen::a_ast::*;
use crate::utilities::error_handler::emission_error;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::io::Error;

pub fn output(ast: AssemFuncDef, name: &str) {
    let path_string = format!("{}.s", name).to_string();
    let path = Path::new(&path_string);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => emission_error(&format!("Cannot open file {}: {}", display, why)),
        Ok(file) => file,
    };

    match write_program(&ast, &mut file) {
        Err(why) => emission_error(&format!("Failed to write Assembly:\n{}", why)),
        Ok(_) => (),
    };

    match file.write(b".section .note.GNU-stack,\"\",@progbits") {
        Err(why) => emission_error(&format!("Failed to write to Assembly:\n{}", why)),
        Ok(_) => (),
    };
}

fn write_program(ast: &AssemFuncDef, file: &mut File) -> Result<(), Error> {
    match ast {
        AssemFuncDef::Function(name, instructions) => {
            file.write_all(format!(".globl {}\n", name).as_bytes())?;
            file.write_all(format!("{}:\n", name).as_bytes())?;
            write_prologue(file)?;
            write_instructions(instructions, file)?;
            Ok(())
        },
    }
}

fn write_instructions(instructions: &Vec<AInstr>, file: &mut File) -> Result<(), Error> {
    for instruction in instructions.iter() {
        write_instruction(instruction, file)?;
    }

    Ok(())
}

fn write_instruction(instruction: &AInstr, file: &mut File) -> Result<(), Error> {
    match instruction {
        AInstr::Mov(left, right) => {
            let src = get_operand(left);
            let dst = get_operand(right);

            file.write_all(format!("\tmovl {}, {}\n", &src, &dst).as_bytes())?;
            Ok(())
        },
        AInstr::Unary(op, oprnd) => {
            let operator = get_operator(op);
            let operand = get_operand(oprnd);

            file.write_all(format!("\t{} {}\n", &operator, &operand).as_bytes())?;
            Ok(())
        },
        AInstr::AllocateStack(val) => {
            file.write_all(format!("\tsubq ${}, %rsp\n", val).as_bytes())?;
            Ok(())
        },
        AInstr::Ret => {
            file.write_all(b"\tmovq %rbp, %rsp\n\tpopq %rbp\n\tret\n")?;
            Ok(())
        },
    }
}

fn get_operand(op: &AOprnd) -> String {
    match op {
        AOprnd::Reg(AReg::AX) => "%eax".to_string(),
        AOprnd::Reg(AReg::R10) => "%r10d".to_string(),
        AOprnd::Stack(val) => format!("{}(%rbp)", val).to_string(),
        AOprnd::Imm(val) => format!("${}", val).to_string(),
        _ => "Should Not Exist".to_string(),
    }
}

fn get_operator(op: &AUnaryOp) -> String {
    match op {
        AUnaryOp::Neg => "negl".to_string(),
        AUnaryOp::Not => "notl".to_string(),
    }
}

fn write_prologue(file: &mut File) -> Result<(), Error> {
    file.write_all(b"\tpushq %rbp\n\tmovq %rsp, %rbp\n")?;
    Ok(())
}
