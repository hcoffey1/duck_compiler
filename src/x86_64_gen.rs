//Hayden Coffey
use std::fs::File;
use std::io::prelude::*;

use crate::instruction::{DuckInstruction, InstructionEnum};

static ARRAY_BASE_REG: &str = "r12";
static DUCK_COUNT_REG: &str = "r13";
static GOOSE_INDEX_REG: &str = "r14";

//Translate given duck index to array index and store in given register
//Uses r8, rbx, rax, rdx
fn get_duck_index(register: &str, duck: usize, file: &mut File) -> std::io::Result<()> {
    get_goose_index("r8", file)?;

    //Add duck to goose number
    writeln!(file, "  add ${}, %r8", duck)?;

    //Mod by duck count + 1
    writeln!(file, "  mov %r8, %rax")?;
    writeln!(file, "  mov $0, %rdx")?;

    writeln!(file, "  mov %{}, %rbx", DUCK_COUNT_REG)?;
    writeln!(file, "  add $1, %rbx")?;

    writeln!(file, "  divq %rbx")?;

    //Move into desired register
    writeln!(file, "  mov %rdx, %{}", register)?;

    Ok(())
}

//Store goose index in given register
fn get_goose_index(register: &str, file: &mut File) -> std::io::Result<()> {
    writeln!(file, "  mov %{}, %{}", GOOSE_INDEX_REG, register)?;

    Ok(())
}

//Store teacher index in given register
fn get_teacher_index(register: &str, file: &mut File) -> std::io::Result<()> {
    writeln!(file, "  mov %{}, %{}", DUCK_COUNT_REG, register)?;
    writeln!(file, "  add $1, %{}", register)?;

    Ok(())
}

//Assembly header lines, allocate stack array and initialize registers
fn write_header(duck_count: usize, file: &mut File) -> std::io::Result<()> {
    write!(file, ".section .text\n.global main\nmain:\n")?;

    //Add registers for goose and teacher and goose pointer
    writeln!(file, "#Allocate ducks on stack")?;
    for _ in 0..duck_count + 3 {
        //write!(file, " xor %{}, %{}\n", REGISTER_MAP[i], REGISTER_MAP[i])?;
        writeln!(file, "  push $0")?;
    }

    writeln!(file, "  mov %rsp, %{}", ARRAY_BASE_REG)?;
    writeln!(file, "  mov ${}, %{}", duck_count, DUCK_COUNT_REG)?;
    writeln!(file, "  mov $0, %{}", GOOSE_INDEX_REG)?;

    Ok(())
}

//Perform exit syscall
fn write_exit(file: &mut File) -> std::io::Result<()> {
    writeln!(file, "#Exit==========")?;
    write!(file, "  mov $60, %rax\n  mov $0, %rdi\n  syscall\n")?;
    Ok(())
}

//Lower given duck instruction to x86
fn write_add(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
    writeln!(file, "#Add==========")?;
    writeln!(file, "#n: {}", inst.n)?;
    writeln!(file, "#y: {}", inst.y)?;
    writeln!(file, "#goose: {}", inst.goose)?;

    //Get n and y index
    get_duck_index("r10", inst.n, file)?;
    get_duck_index("r11", inst.y, file)?;

    //Load n and y
    writeln!(file, "  movq (%{}, %r10, 8),%r8", ARRAY_BASE_REG)?;
    writeln!(file, "  movq (%{}, %r11, 8),%r9", ARRAY_BASE_REG)?;

    //Add n and y
    writeln!(file, "  add %r8, %r9")?;

    //Move N -> Goose
    get_goose_index("rax", file)?;
    writeln!(file, "  movq %r9, (%{}, %rax, 8)", ARRAY_BASE_REG)?;

    //Update goose index
    writeln!(file, "  mov %r10, %{}", GOOSE_INDEX_REG)?;

    Ok(())
}

fn write_subtract(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
    writeln!(file, "#Subtract==========")?;
    writeln!(file, "#n: {}", inst.n)?;
    writeln!(file, "#y: {}", inst.y)?;
    writeln!(file, "#goose: {}", inst.goose)?;

    //Get n and y index
    get_duck_index("r10", inst.n, file)?;
    get_duck_index("r11", inst.y, file)?;

    //Load n and y
    writeln!(file, "  movq (%{}, %r10, 8),%r8", ARRAY_BASE_REG)?;
    writeln!(file, "  movq (%{}, %r11, 8),%r9", ARRAY_BASE_REG)?;

    //Subtract n and y (n - y)
    writeln!(file, "  sub %r9, %r8")?;

    //Move N -> Goose
    get_goose_index("rax", file)?;
    writeln!(file, "  movq %r8, (%{}, %rax, 8)", ARRAY_BASE_REG)?;

    //Update goose index
    writeln!(file, "  mov %r10, %{}", GOOSE_INDEX_REG)?;

    Ok(())
}

fn write_multiply(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
    writeln!(file, "#Multiply==========")?;
    writeln!(file, "#n: {}", inst.n)?;
    writeln!(file, "#y: {}", inst.y)?;
    writeln!(file, "#goose: {}", inst.goose)?;

    //Get n and y index
    get_duck_index("r10", inst.n, file)?;
    get_duck_index("r11", inst.y, file)?;

    //Load n and y
    writeln!(file, "  movq (%{}, %r10, 8),%r8", ARRAY_BASE_REG)?;
    writeln!(file, "  movq (%{}, %r11, 8),%r9", ARRAY_BASE_REG)?;

    //Multiply n and y
    writeln!(file, "  imul %r8, %r9")?;

    //Move N -> Goose
    get_goose_index("rax", file)?;
    writeln!(file, "  movq %r9, (%{}, %rax, 8)", ARRAY_BASE_REG)?;

    //Update goose index
    writeln!(file, "  mov %r10, %{}", GOOSE_INDEX_REG)?;

    Ok(())
}
fn write_divide(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
    writeln!(file, "#Divide==========")?;
    writeln!(file, "#n: {}", inst.n)?;
    writeln!(file, "#y: {}", inst.y)?;
    writeln!(file, "#goose: {}", inst.goose)?;

    //Get n and y index
    get_duck_index("r10", inst.n, file)?;
    get_duck_index("r11", inst.y, file)?;

    //Load n and y
    writeln!(file, "  movq (%{}, %r10, 8),%r8", ARRAY_BASE_REG)?;
    writeln!(file, "  movq (%{}, %r11, 8),%r9", ARRAY_BASE_REG)?;

    //Divide n/y
    writeln!(file, "  mov $0, %rdx")?;
    writeln!(file, "  mov %r8, %rax")?;

    writeln!(file, "  divq %r9")?;

    //Move N -> Goose
    get_goose_index("rbx", file)?;
    writeln!(file, "  movq %rax, (%{}, %rbx, 8)", ARRAY_BASE_REG)?;

    //Update goose index
    writeln!(file, "  mov %r10, %{}", GOOSE_INDEX_REG)?;

    Ok(())
}

//Reads a char from stdin
fn write_input(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
    writeln!(file, "#Input==========")?;
    //Allocate space on the stack
    writeln!(file, "  sub $8, %rsp")?;

    //rax <- 0 (syscall number for 'read')
    writeln!(file, "  xor %eax, %eax")?;
    //edi <- 0 (stdin file descriptor)
    writeln!(file, "  xor %edi, %edi")?;
    //rsi <- address of the buffer.  lea rsi, [rel buffer]
    writeln!(file, "  mov %rsp, %rsi")?;
    //rdx <- size of the buffer
    writeln!(file, "  mov $1, %edx")?;

    writeln!(file, "  syscall")?;

    get_duck_index("r10", inst.n, file)?;
    get_goose_index("rax", file)?;

    writeln!(file, "  pop %r11")?;

    writeln!(file, "  movq %r11, (%{}, %rax, 8)", ARRAY_BASE_REG)?;

    writeln!(file, "  mov %r10, %{}", GOOSE_INDEX_REG)?;

    Ok(())
}

//Push value to teacher
fn write_push(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
    get_teacher_index("r11", file)?;
    get_duck_index("r10", inst.n, file)?;
    writeln!(file, "  movq (%{}, %r10, 8),%r8", ARRAY_BASE_REG)?;

    //Move N -> Goose
    get_goose_index("rax", file)?;
    writeln!(file, "  movq %r8, (%{}, %rax, 8)", ARRAY_BASE_REG)?;

    //Move N -> Teacher
    writeln!(file, "  movq %r8, (%{}, %r11, 8)", ARRAY_BASE_REG)?;

    //Update goose index
    writeln!(file, "  mov %r10, %{}", GOOSE_INDEX_REG)?;

    Ok(())
}

//Pop value from teacher to goose
fn write_pop(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
    //Load teacher
    get_teacher_index("r11", file)?;
    writeln!(file, "  movq (%{}, %r11, 8),%r8", ARRAY_BASE_REG)?;

    //Move teacher -> Goose
    get_goose_index("rax", file)?;
    writeln!(file, "  movq %r8, (%{}, %rax, 8)", ARRAY_BASE_REG)?;

    //Set teacher to 0
    writeln!(file, "  movq $0, (%{}, %r11, 8)", ARRAY_BASE_REG)?;

    //Update goose index
    get_duck_index("r10", inst.n, file)?;
    writeln!(file, "  mov %r10, %{}", GOOSE_INDEX_REG)?;

    Ok(())
}

fn write_loop_begin(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
    writeln!(file, "#LoopBegin==========")?;
    writeln!(file, "  start_{}:", inst.y)?;

    get_duck_index("r10", inst.n, file)?;
    writeln!(file, "  movq (%{}, %r10, 8),%r8", ARRAY_BASE_REG)?;
    writeln!(file, "  cmp $0, %r8")?;
    writeln!(file, "  jz end_{}", inst.y)?;
    Ok(())
}

fn write_loop_end(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
    writeln!(file, "#LoopEnd==========")?;
    writeln!(file, "  jmp start_{}", inst.y)?;
    writeln!(file, "  end_{}:", inst.y)?;
    Ok(())
}

fn write_set(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
    writeln!(file, "#Set==========")?;
    writeln!(file, "#n: {}", inst.n)?;
    writeln!(file, "#y: {}", inst.y)?;
    writeln!(file, "#goose: {}", inst.goose)?;

    get_duck_index("r10", inst.n, file)?;
    get_goose_index("rax", file)?;

    writeln!(file, "  movq ${}, (%{}, %rax, 8)", inst.y, ARRAY_BASE_REG)?;

    writeln!(file, "  mov %r10, %{}", GOOSE_INDEX_REG)?;
    Ok(())
}

fn write_print(inst: &DuckInstruction, file: &mut File) -> std::io::Result<()> {
    writeln!(file, "#Print==========")?;

    get_duck_index("r10", inst.n, file)?;
    writeln!(file, "  movq (%{}, %r10, 8),%r8", ARRAY_BASE_REG)?;

    writeln!(file, "  push %r8")?;

    //sys_write call number
    writeln!(file, "  mov $1, %rax")?;
    //write to stdout (fd=1)
    writeln!(file, "  mov $1, %rdi")?;
    //use char on stack
    writeln!(file, "  mov %rsp, %rsi")?;
    //write 1 char
    writeln!(file, "  mov $1, %rdx")?;
    writeln!(file, "  syscall")?;
    writeln!(file, "  add $8, %rsp")?;

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
