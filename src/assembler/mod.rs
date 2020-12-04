use nom::types::CompleteStr;

use vm::VM;
use assembler::program_parsers::program;
use assembler::program_parsers::Program;
use instruction::Opcode;


pub mod opcode_parsers;
pub mod operand_parsers;
pub mod register_parsers;
pub mod instruction_parsers;
pub mod program_parsers;
pub mod label_parsers;
pub mod directive_parsers;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op{code: Opcode},
    Register{reg_num: u8},
    IntegerOperand{value: i32},
    LabelDeclaration{name: String},
    LabelUsage{name: String },
    Directive{name: String },
}

#[derive(Debug)]
enum AssemblerPhase {
    First,
    Second,
}

#[derive(Debug)]
pub struct Assembler {
    phase: AssemblerPhase,
    pub symbols: SymbolTable,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new(),
        }
    }

    pub fn assemble(&mut self, raw: &str) -> Option<Vec<u8>> {
        match program(CompleteStr(raw)) {
            Ok((_remainder, program)) => {
                self.process_first_phase(&program);
                Some(self.process_second_phase(&program))
            },
            Err(e) => {
                println!("There was an error assembling the code: {:?}", e);
                None
            }
        }
    }

    fn process_first_phase(&mut self, p: &Program) {
        self.extract_labels(p);
        self.phase = AssemblerPhase::Second;
    }

    fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
        let mut program = vec![];
        for i in &p.instructions {
            let mut bytes = i.to_bytes(&self.symbols);
            program.append(&mut bytes);
        }
        program
    }

    fn extract_labels(&mut self, p: &Program) {
        let mut c = 0;
        for i in &p.instructions {
            if i.is_label() {
                match i.get_label_name() {
                    Some(name) => {
                        let symbol = Symbol::new(name, SymbolType::Label, c);
                        self.symbols.add_symbol(symbol);
                    },
                    None => {}
                }
            }
            c += 4;
        }
    }
}

/*
#[test]
fn test_assemble_program() {
    let mut asm = Assembler::new();
    let test_string = r"
        load $0 #100
        load $1 #1
        load $2 #0
        test: inc $0
        neq $0 $2
        jmpe @test
        hlt
        ";
    let program = asm.assemble(test_string).unwrap();
    println!("{:?}", program);
    assert_eq!(program.len(), 21);

    let mut vm = VM::new();
    vm.add_bytes(program);
    assert_eq!(vm.program.len(), 21);
}
*/

#[derive(Debug)]
pub struct Symbol {
    name: String,
    offset: u32,
    symbol_type: SymbolType,
}

impl Symbol {
    pub fn new(name: String, symbol_type: SymbolType, offset: u32) -> Symbol {
        Symbol{
            name,
            symbol_type,
            offset,
        }
    }
}

#[derive(Debug)]
pub enum SymbolType {
    Label,
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: Vec<Symbol>
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable{
            symbols: vec![],
        }
    }

    pub fn add_symbol(&mut self, s: Symbol) {
        self.symbols.push(s);
    }

    pub fn symbol_value(&self, s: &str) -> Option<u32> {
        for symbol in &self.symbols {
            if symbol.name == s {
                return Some(symbol.offset);
            }
        }
        None
    }
}

#[test]
fn test_symbol_table() {
    let mut sym = SymbolTable::new();
    let new_symbol = Symbol::new("test".to_string(), SymbolType::Label, 12);
    sym.add_symbol(new_symbol);
    assert_eq!(sym.symbols.len(), 1);
    let v = sym.symbol_value("test");
    assert_eq!(v.is_some(), true);
    let v = v.unwrap();
    assert_eq!(v, 12);
    let v = sym.symbol_value("does_not_exist");
    assert_eq!(v.is_some(), false);
}

