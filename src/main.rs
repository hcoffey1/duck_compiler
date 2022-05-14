//Hayden Coffey
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

mod parse;

fn main() {
    //Get input file path
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: {} file.duck", args[0]);
        return;
    }

    //Open file and init reader
    let path = Path::new(&args[1]);
    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let mut reader = BufReader::new(file);

    let parse_results = parse::parse_file(&mut reader);

    println!("There are {} duck(s).", parse_results.0);
    for inst in parse_results.1
    {
        println!("{}: {},{}", inst.op_code, inst.n, inst.y);
    }

}

//x if x == Instruction::End as usize => println!("End"),
//x if x == Instruction::Print as usize => println!("Print {}", op1.unwrap()),
//x if x == Instruction::Add as usize => println!("Add {} {}", op1.unwrap(), op2.unwrap()),
//x if x == Instruction::Subtract as usize => println!("Subtract"),
//x if x == Instruction::Multiply as usize => println!("Multiply"),
//x if x == Instruction::Divide as usize => println!("Divide"),
//x if x == Instruction::Input as usize => println!("Input"),
//x if x == Instruction::Push as usize => println!("Push"),
//x if x == Instruction::Pop as usize => println!("Pop"),
//x if x == Instruction::LoopBegin as usize => println!("LoopBegin"),
//x if x == Instruction::LoopEnd as usize => println!("LoopEnd"),
//x if x == Instruction::Set as usize => println!("Set"),
//x => panic!("Unhandled instruction code {}", x),
