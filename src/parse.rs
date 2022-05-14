use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

enum InstructionEnum {
    End,
    Print,
    Add,
    Subtract,
    Multiply,
    Divide,
    Input,
    Push,
    Pop,
    LoopBegin,
    LoopEnd,
    Set,
}

pub struct DuckInstruction {
    pub op_code: usize,
    pub n: usize,
    pub y: usize,
    pub arg_c: usize,
}

fn get_counts(line: &str) -> (usize, usize) {
    let c_pos = line.find('#');

    let line = if c_pos == None {
        line
    } else {
        &line[..c_pos.unwrap()]
    };

    let duck_count = line.matches("duck").count();
    let goose_count = line.matches("goose").count();

    (duck_count, goose_count)
}

fn check_line(line: &str, goose_count: usize) {
    if goose_count > 1 {
        panic!("There can only be one goose!");
    }

    let goose_pos = line.find("goose").unwrap();
    let duck_pos = match line.rfind("duck") {
        None => 0,
        Some(pos) => pos,
    };

    if duck_pos > goose_pos {
        panic!("Duck after goose!");
    }
}

fn parse_header_line(line: &str) -> (usize, usize) {
    let counts = get_counts(line);

    if counts.1 != 0 {
        check_line(line, counts.1);
    } else if counts.0 != 0 {
        panic!("Missing goose!");
    }

    counts
}

fn parse_header(reader: &mut BufReader<File>) -> (usize, usize) {
    loop {
        let mut s = String::new();
        match reader.read_line(&mut s) {
            Err(why) => panic!("{}", why),
            Ok(_) => (),
        };

        let counts = parse_header_line(&s);
        if counts.1 != 0 {
            return counts;
        }
    }
}

fn parse_binary_inst(inst: usize, operands: &mut Vec<usize>) -> DuckInstruction {
    let y = match operands.pop() {
        None => panic!("Mismatched argument count! Inst: {}", inst),
        Some(op) => op,
    };
    let n = match operands.pop() {
        None => panic!("Mismatched argument count! Inst: {}", inst),
        Some(op) => op,
    };

    DuckInstruction {
        op_code: inst,
        n,
        y,
        arg_c: 2,
    }
}

fn parse_unary_inst(inst: usize, operands: &mut Vec<usize>) -> DuckInstruction {
    let n = match operands.pop() {
        None => panic!("Mismatched argument count! Inst: {}", inst),
        Some(op) => op,
    };

    DuckInstruction {
        op_code: inst,
        n,
        y: 0,
        arg_c: 1,
    }
}

fn parse_loop_inst(inst: usize, operands: &mut Vec<usize>) -> DuckInstruction {
    if inst == InstructionEnum::LoopBegin as usize {
        let y = match operands.pop() {
            None => 0,
            Some(op) => op,
        };
        let n = match operands.pop() {
            None => panic!("Mismatched loop argument count! Inst: {}", inst),
            Some(op) => op,
        };

        DuckInstruction {
            op_code: inst,
            n,
            y,
            arg_c: 2,
        }
    } else {
        let n = match operands.pop() {
            None => 0,
            Some(op) => op,
        };
        DuckInstruction {
            op_code: inst,
            n,
            y: 0,
            arg_c: 2,
        }
    }
}

pub fn parse_file(reader: &mut BufReader<File>) -> (usize, Vec<DuckInstruction>) {
    //Read file header
    let counts = parse_header(reader);

    //Parse and record program body instructions
    let mut ops = Vec::<usize>::new();
    let mut duck_inst = Vec::<DuckInstruction>::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let counts = get_counts(&line);

        if counts.1 == 0 {
            ops.push(counts.0);
        } else {
            match counts.0 {
                x if x == InstructionEnum::Print as usize
                    || x == InstructionEnum::Input as usize
                    || x == InstructionEnum::Push as usize
                    || x == InstructionEnum::Pop as usize =>
                {
                    duck_inst.push(parse_unary_inst(x, &mut ops));
                }

                x if x == InstructionEnum::Add as usize
                    || x == InstructionEnum::Subtract as usize
                    || x == InstructionEnum::Multiply as usize
                    || x == InstructionEnum::Divide as usize
                    || x == InstructionEnum::Set as usize =>
                {
                    duck_inst.push(parse_binary_inst(x, &mut ops));
                }

                x if x == InstructionEnum::LoopBegin as usize
                    || x == InstructionEnum::LoopEnd as usize =>
                {
                    duck_inst.push(parse_loop_inst(x, &mut ops));
                }

                x if x == InstructionEnum::End as usize => duck_inst.push(DuckInstruction {
                    op_code: x,
                    n: 0,
                    y: 0,
                    arg_c: 0,
                }),

                x => panic!("Unhandled instruction code {}", x),
            }
            ops.clear();
        }
    }

    if duck_inst.last().unwrap().op_code != InstructionEnum::End as usize {
        panic!("Program does not end with goose!");
    }

    (counts.0, duck_inst)
}
