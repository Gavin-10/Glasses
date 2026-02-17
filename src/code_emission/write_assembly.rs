
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
        },
        AInstr::Unary(op, oprnd) => {
            let operator = get_unary_operator(op);
            let operand = get_operand(oprnd);

            file.write_all(format!("\t{} {}\n", &operator, &operand).as_bytes())?;
        },
        AInstr::Binary(op, left, right) => {
            let operator = get_binary_operator(op);
            let src = get_operand(left);
            let dst = get_operand(right);
            
            file.write_all(format!("\t{} {}, {}\n", &operator, &src, &dst).as_bytes())?;
        },
        AInstr::Idiv(op) => {
            let operand = get_operand(op);

            file.write_all(format!("\tidivl {}\n", operand).as_bytes())?;
        },
        AInstr::Cdq => {
            file.write_all("\tcdq\n".as_bytes())?;
        },
        AInstr::AllocateStack(val) => {
            file.write_all(format!("\tsubq ${}, %rsp\n", val).as_bytes())?;
        },
        AInstr::Ret => {
            file.write_all(b"\tmovq %rbp, %rsp\n\tpopq %rbp\n\tret\n")?;
        },
        AInstr::Cmp(op1, op2) => {
            let left = get_operand(&op1);
            let right = get_operand(&op2);
            file.write_all(format!("\tcmpl {}, {}\n", left, right).as_bytes())?;
        },
        AInstr::Jmp(label) => {
            let label_out = get_label(&label);
            file.write_all(format!("\tjmp {}\n", label_out).as_bytes())?;
        },
        AInstr::JmpCC(code, label) => {
            let cond_code = get_cond_code(code);
            let label_out = get_label(&label);
            file.write_all(format!("\tj{} {}\n", cond_code, label_out).as_bytes())?;
        },
        AInstr::SetCC(code, op) => {
            let cond_code = get_cond_code(&code);
            let op_out = get_byte_operand(&op);
            file.write_all(format!("\tset{} {}\n", cond_code, op_out).as_bytes())?;
        },
        AInstr::Label(label) => {
            let label_out = get_label(&label);
            file.write_all(format!("{}:\n", label_out).as_bytes())?;
        }
    };
    Ok(())
}

fn get_label(op: &String) -> String {
    let mut label = String::from(".L");
    label += op;
    label
}

fn get_cond_code(code: &CondCode) -> String {
    match code {
        CondCode::E => "e".to_string(),
        CondCode::NE => "ne".to_string(),
        CondCode::L => "l".to_string(),
        CondCode::LE => "le".to_string(),
        CondCode::G => "g".to_string(),
        CondCode::GE => "ge".to_string(),
    }
}

fn get_byte_operand(op: &AOprnd) -> String {
    match op {
        AOprnd::Reg(AReg::AX) => "%al".to_string(),
        AOprnd::Reg(AReg::DX) => "%dl".to_string(),
        AOprnd::Reg(AReg::R10) => "%r10b".to_string(),
        AOprnd::Reg(AReg::R11) => "%r11b".to_string(),
        _ => get_operand(op),
    }
}

fn get_operand(op: &AOprnd) -> String {
    match op {
        AOprnd::Reg(AReg::AX) => "%eax".to_string(),
        AOprnd::Reg(AReg::DX) => "%edx".to_string(),
        AOprnd::Reg(AReg::R10) => "%r10d".to_string(),
        AOprnd::Reg(AReg::R11) => "%r11d".to_string(),
        AOprnd::Stack(val) => format!("{}(%rbp)", val).to_string(),
        AOprnd::Imm(val) => format!("${}", val).to_string(),
        _ => "Should Not Exist".to_string(),
    }
}

fn get_unary_operator(op: &AUnaryOp) -> String {
    match op {
        AUnaryOp::Neg => "negl".to_string(),
        AUnaryOp::Not => "notl".to_string(),
    }
}

fn get_binary_operator(op: &ABinaryOp) -> String {
    match op {
        ABinaryOp::Add => "addl".to_string(),
        ABinaryOp::Sub => "subl".to_string(),
        ABinaryOp::Mult => "imull".to_string(),
    }
}

fn write_prologue(file: &mut File) -> Result<(), Error> {
    file.write_all(b"\tpushq %rbp\n\tmovq %rsp, %rbp\n")?;
    Ok(())
}
