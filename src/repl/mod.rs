use std;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use vm::VM;

use assembler::Assembler;
use assembler::program_parsers::program;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to Iridium! Let's be productive!");
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            stdin.read_line(&mut buffer).expect("Unable to read line from user");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                },
                ".load_file" => {
                    print!("Please enter the path to the file you wish to load: ");
                    io::stdout().flush().expect("Unable to flush stdout");
                    let mut tmp = String::new();
                    stdin.read_line(&mut tmp).expect("Unable to read line from user");
                    let tmp = tmp.trim();
                    let filename = Path::new(&tmp);
                    let mut f = File::open(Path::new(&filename)).expect("File not found");
                    let mut contents = String::new();
                    f.read_to_string(&mut contents).expect("There was an error reading from the file");
                    let mut asm = Assembler::new();
                    if let Some(mut bytes) = asm.assemble(&contents) {
                        self.vm.program.append(&mut bytes);
                    }
                },
                ".program" => {
                    println!("Listing instructions currently in VM's program vector:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("End of Program Listing");
                },
                ".registers" => {
                    println!("Listing registers and all content:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of Register Listing")
                },
                ".quit" => {
                    println!("Farewall! Have a great day!");
                    std::process::exit(0);
                },
                _ => {
                    let program = match program(buffer.into()) {
                        Ok((_, program)) => program,
                        Err(_) => {
                            println!("Unable to parse input");
                            continue;
                        }
                    };
                    let asm = Assembler::new();
                    self.vm.program.append(&mut program.to_bytes(&asm.symbols));
                    self.vm.run_once();
                }
            }
        }
    }
}
