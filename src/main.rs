//Hayden Coffey
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::Command;

use gdd::{parse, x86_64_gen};

fn get_file_name(path: &str) -> &str {
    let pos_end = path.find(".ddg").unwrap();
    let pos_begin = path.rfind("/").unwrap() + 1;

    &path[pos_begin..pos_end]
}

fn main() -> Result<(), Error> {
    //Get input file path
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: {} file.duck", args[0]);

        return Err(Error::new(ErrorKind::InvalidInput, "Missing target file."));
    }

    //Open file and init reader
    let path = Path::new(&args[1]);

    let file_name_base = get_file_name(&args[1]);

    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let mut reader = BufReader::new(file);

    //Parse input file and create IR
    let parse_results = parse::parse_file(&mut reader);

    //Transform IR to x86_64
    let file_name_asm = format!("{}.s", file_name_base);
    x86_64_gen::lower_program(&parse_results, &file_name_asm)?;

    //Kinda cheating and using gcc to assemble for now
    //Also, this is probably a security vulnerability
    let output_arg = format!("-o{}", file_name_base);
    Command::new("gcc")
        .args([file_name_asm, String::from("-g"), output_arg])
        .spawn()
        .expect("Failed to assemble program.");

    Ok(())
}
