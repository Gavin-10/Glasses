
use crate::code_gen::assembly_ast::*;
use crate::utilities::error_handler::emission_error;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::io::Error;

pub fn output(ast: AssemFunctionDef, name: &str) {
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

fn write_program(ast: &AssemFunctionDef, file: &mut File) -> Result<(), Error> {
    match ast {
        AssemFunctionDef::Function(name, instructions) => {
            file.write_all(format!(".globl {}\n", name).as_bytes())?;
            file.write_all(format!("{}:\n", name).as_bytes())?;
            write_instructions(instructions, file)?;
            Ok(())
        },
    }
}

fn write_instructions(instructions: &Vec<Instruction>, file: &mut File) -> Result<(), Error> {
    for instruction in instructions.iter() {
        write_instruction(instruction, file)?;
    }

    Ok(())
}

fn write_instruction(instruction: &Instruction, file: &mut File) -> Result<(), Error> {
    match instruction {
        Instruction::Mov(left, right) => {
            let src = get_operand(left);
            let dst = get_operand(right);

            file.write_all(format!("\tmovl {}, {}\n", &src, &dst).as_bytes())?;
            Ok(())
        },
        Instruction::Ret => {
            file.write(b"\tret\n")?;
            Ok(())
        },
    }
}

fn get_operand(op: &Operand) -> String {
    match op {
        Operand::Register => "%eax".to_string(),
        Operand::Imm(val) => format!("${}", val).to_string(),
    }
}
