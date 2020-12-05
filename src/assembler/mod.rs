pub mod assembler_errors;
pub mod opcode_parsers;
pub mod operand_parsers;
pub mod register_parsers;
pub mod instruction_parsers;
pub mod program_parsers;
pub mod label_parsers;
pub mod directive_parsers;
pub mod symbols;

use nom::types::CompleteStr;

use assembler::assembler_errors::AssemblerError;
use assembler::instruction_parsers::AssemblerInstruction;
use assembler::program_parsers::program;
use assembler::program_parsers::Program;
use assembler::symbols::Symbol;
use assembler::symbols::SymbolTable;
use assembler::symbols::SymbolType;
use instruction::Opcode;

pub const PIE_HEADER_PREFIX: [u8; 4] = [0x45, 0x50, 0x49, 0x45];
pub const PIE_HEADER_LENGTH: usize = 64;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op{code: Opcode},
    Register{reg_num: u8},
    IntegerOperand{value: i32},
    LabelDeclaration{name: String},
    LabelUsage{name: String },
    Directive{name: String },
    IrString{name: String},
}

#[derive(Debug, PartialEq)]
enum AssemblerPhase {
    First,
    Second,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssemblerSection {
    Data { starting_instruction: Option<u32> },
    Code { starting_instruction: Option<u32> },
    Unknown,
}

impl<'a> From<&'a str> for AssemblerSection {
    fn from(name: &str) -> AssemblerSection {
        match name {
            "data" => AssemblerSection::Data { starting_instruction: None },
            "code" => AssemblerSection::Code { starting_instruction: None },
            _ => AssemblerSection::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct Assembler {
    phase: AssemblerPhase,
    pub symbols: SymbolTable,
    pub ro: Vec<u8>,
    pub bytecode: Vec<u8>,
    ro_offset: u32,
    sections: Vec<AssemblerSection>,
    current_section: Option<AssemblerSection>,
    current_instruction: u32,
    errors: Vec<AssemblerError>,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new(),
            ro: vec![],
            bytecode: vec![],
            ro_offset: 0,
            sections: vec![],
            current_section: None,
            current_instruction: 0,
            errors: vec![],
        }
    }

    pub fn assemble(&mut self, raw: &str) -> Result<Vec<u8>, Vec<AssemblerError>> {
        match program(CompleteStr(raw)) {
            Ok((_remainder, program)) => {
                let mut assembled_program = self.write_pie_header();
                self.process_first_phase(&program);
                if !self.errors.is_empty() {
                    return Err(self.errors.clone());
                }

                if self.sections.len() != 2 {
                    println!("Did not find at least two sections.");
                    self.errors.push(AssemblerError::InsufficientSections);
                    return Err(self.errors.clone());
                }

                let mut body = self.process_second_phase(&program);

                assembled_program.append(&mut body);
                Ok(assembled_program)
            },
            Err(e) => {
                println!("There was an error assembling the code: {:?}", e);
                Err(vec![AssemblerError::ParseError{ error: e.to_string() }])
            }
        }
    }

    fn process_first_phase(&mut self, p: &Program) {
        for i in &p.instructions {
            if i.is_label() {
                if self.current_section.is_some() {
                    self.process_label_declaration(&i);
                } else {
                    self.errors.push(AssemblerError::NoSegmentDeclarationFound{instruction: self.current_instruction});
                }
            }

            if i.is_directive() {
                self.process_directive(i);
            }

            self.current_instruction += 1;
        }

        self.phase = AssemblerPhase::Second;
    }

    fn process_label_declaration(&mut self, i: &AssemblerInstruction) {
        let name = match i.get_label_name() {
            Some(name) => { name },
            None => {
                self.errors.push(AssemblerError::StringConstantDeclaredWithoutLabel{instruction: self.current_instruction});
                return;
            }
        };

        if self.symbols.has_symbol(&name) {
            self.errors.push(AssemblerError::SymbolAlreadyDeclared);
            return;
        }

        let symbol = Symbol::new(name, SymbolType::Label);
        self.symbols.add_symbol(symbol);
    }

    fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
        self.current_instruction = 0;

        let mut program = vec![];
        for i in &p.instructions {
            if i.is_opcode() {
                let mut bytes = i.to_bytes(&self.symbols);
                program.append(&mut bytes);
            }
            if i.is_directive() {
                self.process_directive(i);
            }
            self.current_instruction += 1;
        }
        program
    }

    fn process_directive(&mut self, i: &AssemblerInstruction) {
        let directive_name = match i.get_directive_name() {
            Some(name) => name,
            None => {
                println!("Directive has an invalid name: {:?}", i);
                return;
            },
        };

        if i.has_operands() {
            match directive_name.as_ref() {
                "asciiz" => {
                    self.handle_asciiz(i);
                }
                _ => {
                    self.errors.push(AssemblerError::UnknownDirectiveFound{directive: directive_name.clone() });
                    return;
                }
            }
        } else {
            self.process_section_header(&directive_name);
        }
    }

    fn process_section_header(&mut self, header_name: &str) {
        let new_section: AssemblerSection = header_name.into();
        if new_section == AssemblerSection::Unknown {
            println!("Found an section header that is unknown: {:#?}", header_name);
            return;
        }
        self.sections.push(new_section.clone());
        self.current_section = Some(new_section);
    }

    fn handle_asciiz(&mut self, i: &AssemblerInstruction) {
        if self.phase != AssemblerPhase::First {
            return;
        }

        match i.get_string_constant() {
            Some(s) => {
                match i.get_label_name() {
                    Some(name) => {
                        self.symbols.set_symbol_offset(&name, self.ro_offset);
                    }
                    None => {
                        println!("Found a string constant with no associated label!");
                        return;
                    }
                };
                for byte in s.as_bytes() {
                    self.ro.push(*byte);
                    self.ro_offset += 1;
                }
                self.ro.push(0);
                self.ro_offset += 1;
            }
            None => {
                println!("String constant following an .asciiz was empty");
            }
        }
    }

    fn write_pie_header(&self) -> Vec<u8> {
        let mut header = vec![];
        for byte in PIE_HEADER_PREFIX.iter() {
            header.push(byte.clone());
        }
        while header.len() < PIE_HEADER_LENGTH {
            header.push(0);
        }
        header
    }
}

mod tests {
    #![allow(unused_imports)]

    use super::Assembler;
    use super::PIE_HEADER_LENGTH;
    use vm::VM;

    #[test]
    fn test_assemble_program() {
        let mut asm = Assembler::new();
        let test_string = r"
            .data
            .code
            load $0 #100
            load $1 #1
            load $2 #0
            test: inc $0
            neq $0 $2
            jmpe @test
            hlt
        ";
        let result = asm.assemble(test_string);
        assert_eq!(result.is_ok(), true);
        let program = result.unwrap();
        assert_eq!(program.len(), 28 + PIE_HEADER_LENGTH);

        let mut vm = VM::new();
        vm.add_bytes(program);
        assert_eq!(vm.program.len(), 28 + PIE_HEADER_LENGTH);
    }
}
