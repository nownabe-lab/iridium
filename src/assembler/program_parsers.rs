use nom::types::CompleteStr;

use assembler::directive_parsers::directive;
use assembler::instruction_parsers::{AssemblerInstruction, instruction};
use assembler::SymbolTable;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instructions {
            program.append(&mut instruction.to_bytes(symbols));
        }
        program
    }
}

named!(pub program<CompleteStr, Program>,
    do_parse!(
        instructions: many1!(alt!(instruction | directive)) >>
        (
            Program {
                instructions: instructions,
            }
        )
    )
);

#[test]
fn test_parse_program() {
    let result = program(CompleteStr("load $0 #100\n"));
    assert_eq!(result.is_ok(), true);
    let (leftover, p) = result.unwrap();
    assert_eq!(leftover, CompleteStr(""));
    assert_eq!(1, p.instructions.len());
}

#[test]
fn test_program_to_bytes() {
    let result = program(CompleteStr("load $0 #100\n"));
    assert_eq!(result.is_ok(), true);
    let (_, program) = result.unwrap();
    let symbols = SymbolTable::new();
    let bytecode = program.to_bytes(&symbols);
    assert_eq!(bytecode.len(), 4);
    println!("{:?}", bytecode);
}

#[test]
fn test_complete_program() {
    let test_program = CompleteStr(r"
        .data
        hello: .asciiz 'Hello everyone!'
        .code
        hlt
    ");
    let result = program(test_program);
    assert_eq!(result.is_ok(), true);
}
