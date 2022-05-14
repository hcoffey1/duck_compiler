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

#[derive(Clone, Copy)]
pub struct DuckInstruction {
    pub op_code: usize,
    pub n: usize,
    pub y: usize,
    pub arg_c: usize,
}

pub fn get_op_name(op_code: usize) -> String {
    match op_code {
        x if x == InstructionEnum::End as usize => String::from("End"),
        x if x == InstructionEnum::Print as usize => String::from("Print"),
        x if x == InstructionEnum::Add as usize => String::from("Add"),
        x if x == InstructionEnum::Subtract as usize => String::from("Subtract"),
        x if x == InstructionEnum::Multiply as usize => String::from("Multiply"),
        x if x == InstructionEnum::Divide as usize => String::from("Divide"),
        x if x == InstructionEnum::Input as usize => String::from("Input"),
        x if x == InstructionEnum::Push as usize => String::from("Push"),
        x if x == InstructionEnum::Pop as usize => String::from("Pop"),
        x if x == InstructionEnum::LoopBegin as usize => String::from("LoopBegin"),
        x if x == InstructionEnum::LoopEnd as usize => String::from("LoopEnd"),
        x if x == InstructionEnum::Set as usize => String::from("Set"),
        x => panic!("Unhandled instruction code {}", x),
    }
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

fn apply_goose_updates(
    duck_count: usize,
    inst_list: &Vec<DuckInstruction>,
) -> Vec<DuckInstruction> {
    //Create copy of instructions to modify with rotations
    let mut rotated_inst_list = inst_list.to_vec();

    //Set up mapping between rotation postion and index
    let mut duck_mapping = Vec::<usize>::new();
    for i in 0..duck_count + 1 {
        duck_mapping.push(i);
    }

    //Rotate instructions
    for inst_pos in 0..inst_list.len() {
        //Determine rotation amount
        if inst_list[inst_pos].op_code != InstructionEnum::Print as usize
            || inst_list[inst_pos].op_code != InstructionEnum::End as usize
            || inst_list[inst_pos].op_code != InstructionEnum::LoopBegin as usize
            || inst_list[inst_pos].op_code != InstructionEnum::LoopEnd as usize
        {
            duck_mapping.rotate_right(inst_list[inst_pos].n);
        } else {
            continue;
        }

        //Rotate subsequent instructions
        for i in inst_pos + 1..rotated_inst_list.len() {
            let op = inst_list[i].op_code;
            let arg_c = inst_list[i].arg_c;

            if op == InstructionEnum::LoopBegin as usize {
                rotated_inst_list[i].n = duck_mapping
                    .iter()
                    .position(|&r| r == inst_list[i].n)
                    .unwrap();
            } else if op == InstructionEnum::LoopEnd as usize {
            } else if op == InstructionEnum::Set as usize {
                rotated_inst_list[i].n = duck_mapping
                    .iter()
                    .position(|&r| r == inst_list[i].n)
                    .unwrap();
            } else if arg_c == 1 {
                rotated_inst_list[i].n = duck_mapping
                    .iter()
                    .position(|&r| r == inst_list[i].n)
                    .unwrap();
            } else if arg_c == 2 {
                rotated_inst_list[i].n = duck_mapping
                    .iter()
                    .position(|&r| r == inst_list[i].n)
                    .unwrap();
                rotated_inst_list[i].y = duck_mapping
                    .iter()
                    .position(|&r| r == inst_list[i].y)
                    .unwrap();
            }
        }
    }

    rotated_inst_list
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

    let duck_inst = apply_goose_updates(counts.0, &mut duck_inst);

    (counts.0, duck_inst)
}
