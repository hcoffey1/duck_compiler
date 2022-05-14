//Hayden Coffey
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use duck_compiler::parse;

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
    for inst in parse_results.1 {
        let inst_name = parse::get_op_name(inst.op_code);
        match inst.arg_c {
            0 => println!("{}:", inst_name),
            1 => println!("{}: {}", inst_name, inst.n),
            2 => println!("{}: {},{}", inst_name, inst.n, inst.y),
            _ => panic!("Invalid argument size!"),
        }
    }
}
