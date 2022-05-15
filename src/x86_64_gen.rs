use std::fs::File;
use std::io::prelude::*;

use crate::instruction::{DuckInstruction, InstructionEnum};

//Not using rsp, rdx, rax, as a variable
static REGISTER_MAP: [&str; 13] = [
    "rbx", "rcx", "rsi", "rdi", "rbp", "r8", "r9", "r10", "r11", "r12", "r13",
    "r14", "r15",
];

fn write_header(duck_count: usize, file: &mut File) -> std::io::Result<()> {
    write!(file, ".section .text\n.global main\nmain:\n")?;

    //Add registers for goose and teacher 
    for i in 0..duck_count + 2 {
        write!(file, "  mov $0, %{}\n", REGISTER_MAP[i])?;
    }

    Ok(())
}

fn write_exit(file: &mut File) -> std::io::Result<()> {
	writeln!(file, "#Exit==========")?;
    write!(file, "  mov $60, %rax\n  mov $0, %rdi\n  syscall\n")?;
    Ok(())
}

fn write_add(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
	writeln!(file, "#Add==========")?;
	writeln!(file, "#n: {}", inst.n)?;
	writeln!(file, "#y: {}", inst.y)?;
	writeln!(file, "#goose: {}", inst.goose)?;
    //Save N
    writeln!(file, "  push %{}", REGISTER_MAP[inst.n])?;
    //N = N + Y
    writeln!(
        file,
        "  add %{}, %{}",
        REGISTER_MAP[inst.y], REGISTER_MAP[inst.n]
    )?;
    //Move N -> Goose
    writeln!(
        file,
        "  mov %{}, %{}",
        REGISTER_MAP[inst.n], REGISTER_MAP[inst.goose]
    )?;
    //Restore N
    writeln!(file, "  pop %{}", REGISTER_MAP[inst.n])?;

    Ok(())
}
fn write_subtract(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
	writeln!(file, "#Subtract==========")?;
	writeln!(file, "#n: {}", inst.n)?;
	writeln!(file, "#y: {}", inst.y)?;
	writeln!(file, "#goose: {}", inst.goose)?;
    //Save N
    writeln!(file, "  push %{}", REGISTER_MAP[inst.n])?;
    //N = N - Y
    writeln!(
        file,
        "  sub %{}, %{}",
        REGISTER_MAP[inst.y], REGISTER_MAP[inst.n]
    )?;
    //Move N -> Goose
    writeln!(
        file,
        "  mov %{}, %{}",
        REGISTER_MAP[inst.n], REGISTER_MAP[inst.goose]
    )?;
    //Restore N
    writeln!(file, "  pop %{}", REGISTER_MAP[inst.n])?;
    Ok(())
}

//fn write_arithmetic_op(inst: &DuckInstruction, file , file: &mut File)
//{
//	let inst_name : = match  inst.op_code {
//        x if x == InstructionEnum::Add as usize => String::from("add"),
//        x if x == InstructionEnum::Subtract as usize => String::from("sub"),
//        x if x == InstructionEnum::Multiply as usize => String::from("imul"),
//        x if x == InstructionEnum::Divide as usize => ,
//        x => panic!("Unhandled instruction code {}", x),
//	};
//
//}
fn write_multiply(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
	writeln!(file, "#Multiply==========")?;
	writeln!(file, "#n: {}", inst.n)?;
	writeln!(file, "#y: {}", inst.y)?;
	writeln!(file, "#goose: {}", inst.goose)?;
    //Save N
    writeln!(file, "  push %{}", REGISTER_MAP[inst.n])?;
    //N = N * Y
    writeln!(
        file,
        "  imul %{}, %{}",
        REGISTER_MAP[inst.y], REGISTER_MAP[inst.n]
    )?;
    //Move N -> Goose
    writeln!(
        file,
        "  mov %{}, %{}",
        REGISTER_MAP[inst.n], REGISTER_MAP[inst.goose]
    )?;
    //Restore N
    writeln!(file, "  pop %{}", REGISTER_MAP[inst.n])?;
    Ok(())
}
fn write_divide(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
	writeln!(file, "#Divide==========")?;
	writeln!(file, "#n: {}", inst.n)?;
	writeln!(file, "#y: {}", inst.y)?;
	writeln!(file, "#goose: {}", inst.goose)?;


	writeln!(file, "  push %rdx")?;
	writeln!(file, "  push %rax")?;

	writeln!(file, "  mov $0, %rdx")?;
	writeln!(file, "  mov %{}, %rax", REGISTER_MAP[inst.n])?;

	writeln!(file, "  divq %{}", REGISTER_MAP[inst.y])?;

	writeln!(file, "  mov %rax, %{}", REGISTER_MAP[inst.goose])?;

	writeln!(file, "  pop %rax")?;
	writeln!(file, "  pop %rdx")?;

    Ok(())
}
fn write_input(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
    Ok(())
}
fn write_push(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
    Ok(())
}
fn write_pop(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
    Ok(())
}
fn write_loop_begin(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
    Ok(())
}
fn write_loop_end(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
    Ok(())
}
fn write_set(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
	writeln!(file, "#Set==========")?;
	writeln!(file, "#n: {}", inst.n)?;
	writeln!(file, "#y: {}", inst.y)?;
	writeln!(file, "#goose: {}", inst.goose)?;

	writeln!(file, "  mov ${}, %{}", inst.y, REGISTER_MAP[inst.goose])?;
    Ok(())
}


//push    $0x21       # '!'
//mov     $1, %rax    # sys_write call number 
//mov     $1, %rdi    # write to stdout (fd=1)
//mov     %rsp, %rsi  # use char on stack
//mov     $1, %rdx    # write 1 char
//syscall   
//add     $8, %rsp    # restore sp 
fn write_print(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
	writeln!(file, "#Print==========")?;
	writeln!(file, "  push %rcx")?;
	writeln!(file, "  push %rdx")?;
	writeln!(file, "  push %rdi")?;
	writeln!(file, "  push %rsi")?;
	writeln!(file, "  push %rsp")?;
	writeln!(file, "  push %r8")?;
	writeln!(file, "  push %r9")?;
	writeln!(file, "  push %r10")?;
	writeln!(file, "  push %r11")?;

	writeln!(file, "  push %{}", REGISTER_MAP[inst.n])?;
	writeln!(file, "  mov $1, %rax")?;
	writeln!(file, "  mov $1, %rdi")?;
	writeln!(file, "  mov %rsp, %rsi")?;
	writeln!(file, "  mov $1, %rdx")?;
	writeln!(file, "  syscall")?;
	writeln!(file, "  add $8, %rsp")?;

	writeln!(file, "  pop %r11")?;
	writeln!(file, "  pop %r10")?;
	writeln!(file, "  pop %r9")?;
	writeln!(file, "  pop %r8")?;
	writeln!(file, "  pop %rsp")?;
	writeln!(file, "  pop %rsi")?;
	writeln!(file, "  pop %rdi")?;
	writeln!(file, "  pop %rdx")?;
	writeln!(file, "  pop %rcx")?;
    Ok(())
}

fn write_instruction(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
    match inst.op_code {
        x if x == InstructionEnum::End as usize => write_exit(file)?,
        x if x == InstructionEnum::Print as usize => write_print(inst, file)?,
        x if x == InstructionEnum::Add as usize => write_add(inst, file)?,
        x if x == InstructionEnum::Subtract as usize => write_subtract(inst, file)?,
        x if x == InstructionEnum::Multiply as usize => write_multiply(inst, file)?,
        x if x == InstructionEnum::Divide as usize => write_divide(inst, file)?,
        x if x == InstructionEnum::Input as usize => write_input(inst, file)?,
        x if x == InstructionEnum::Push as usize => write_push(inst, file)?,
        x if x == InstructionEnum::Pop as usize => write_pop(inst, file)?,
        x if x == InstructionEnum::LoopBegin as usize => write_loop_begin(inst, file)?,
        x if x == InstructionEnum::LoopEnd as usize => write_loop_end(inst, file)?,
        x if x == InstructionEnum::Set as usize => write_set(inst, file)?,
        x => panic!("Unhandled instruction code {}", x),
    };

    Ok(())
}

pub fn lower_program(
    parse_results: &(usize, Vec<DuckInstruction>),
    file_out_name: &str,
) -> std::io::Result<()> {
    let mut file = File::create(file_out_name)?;
    write_header(parse_results.0, &mut file)?;

    for inst in &parse_results.1 {
        write_instruction(inst, &mut file)?;
    }

    write_exit(&mut file)?;
    Ok(())
}
