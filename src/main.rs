use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

enum Instruction {
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: {} file.duck", args[0]);
        return;
    }

    let path = Path::new(&args[1]);

    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut reader = BufReader::new(file);

    let counts = parse_header(&mut reader);

    println!("There are {} ducks.", counts.0);
    //#reader.read_line(buf)

    for line in reader.lines() {
        let line = line.unwrap();
        let counts = get_counts(&line);

        if counts.1 != 0 {
            match counts.0 {
                x if x == Instruction::End as usize => println!("End"),
                x if x == Instruction::Print as usize => println!("Print"),
                x if x == Instruction::Add as usize => println!("Add"),
                x if x == Instruction::Subtract as usize => println!("Subtract"),
                x if x == Instruction::Multiply as usize => println!("Multiply"),
                x if x == Instruction::Divide as usize => println!("Divide"),
                x if x == Instruction::Input as usize => println!("Input"),
                x if x == Instruction::Push as usize => println!("Push"),
                x if x == Instruction::Pop as usize => println!("Pop"),
                x if x == Instruction::LoopBegin as usize => println!("LoopBegin"),
                x if x == Instruction::LoopEnd as usize => println!("LoopEnd"),
                x if x == Instruction::Set as usize => println!("Set"),
                x => panic!("Unhandled instruction code {}", x),
            }
        }
    }
}
