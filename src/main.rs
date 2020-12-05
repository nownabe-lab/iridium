use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate nom;

use clap::App;

pub mod instruction;
pub mod vm;
pub mod repl;
pub mod assembler;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let target_file = matches.value_of("INPUT_FILE");
    match target_file {
        Some(filename) => {
            let program = read_file(filename);
            let mut asm = assembler::Assembler::new();
            let mut vm = vm::VM::new();
            let program = asm.assemble(&program);
            match program {
                Some(p) => {
                    vm.add_bytes(p);
                    vm.run();
                    std::process::exit(0);
                },
                None => {},
            }
        },
        None => start_repl(),
    }
}

fn start_repl() {
    let mut repl = repl::REPL::new();
    repl.run();
}

fn read_file(filename: &str) -> String {
    let filepath = Path::new(filename);
    match File::open(Path::new(&filepath)) {
        Ok(mut fh) => {
            let mut contents = String::new();
            match fh.read_to_string(&mut contents) {
                Ok(_) => {
                    return contents;
                },
                Err(e) => {
                    println!("There was an error reading file: {:?}", e);
                    std::process::exit(1);
                },
            }
        },
        Err(e) => {
            println!("File not found: {:?}", e);
            std::process::exit(1);
        },
    }
}
