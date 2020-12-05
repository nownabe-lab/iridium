use nom::multispace;
use nom::types::CompleteStr;

use assembler::SymbolTable;
use assembler::Token;
use assembler::opcode_parsers::opcode;
use assembler::operand_parsers::operand;
use assembler::register_parsers::register;
use assembler::label_parsers::label_declaration;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub opcode: Option<Token>,
    pub label: Option<Token>,
    pub directive: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            instruction_combined
        ) >>
        (
            ins
        )
    )
);

named!(instruction_combined<CompleteStr, AssemblerInstruction>,
    ws!(
        do_parse!(
            l: opt!(label_declaration) >>
            o: opcode >>
            o1: opt!(operand) >>
            o2: opt!(operand) >>
            o3: opt!(operand) >>
            (
                AssemblerInstruction{
                    opcode: Some(o),
                    label: l,
                    directive: None,
                    operand1: o1,
                    operand2: o2,
                    operand3: o3,
                }
            )
        )
    )
);

named!(instruction_one<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        opt!(multispace) >>
        (
            AssemblerInstruction{
                opcode: Some(o),
                label: None,
                directive: None,
                operand1: None,
                operand2: None,
                operand3: None,
            }
        )
    )
);

named!(instruction_two<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        r: register >>
        i: operand >>
        (
            AssemblerInstruction{
                opcode: Some(o),
                label: None,
                directive: None,
                operand1: Some(r),
                operand2: Some(i),
                operand3: None,
            }
        )
    )
);

impl AssemblerInstruction {
    pub fn to_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
        let mut results = vec![];
        let ref token = self.opcode;
        match token {
            Some(Token::Op { code }) => match code {
                _ => {
                    let b: u8 = (*code).into();
                    results.push(b);
                }
            },
            _ => {
                println!("Non-opcode found in opcode field");
                std::process::exit(1);
            },
        }

        for operand in &[&self.operand1, &self.operand2, &self.operand3] {
            if let Some(token) = operand {
                AssemblerInstruction::extract_operand(token, &mut results, symbols);
            }
        }

        while results.len() < 4 {
            results.push(0);
        }

        results
    }

    pub fn is_label(&self) -> bool {
        self.label.is_some()
    }

    pub fn get_label_name(&self) -> Option<String> {
        match &self.label {
            Some(l) => match l {
                Token::LabelDeclaration { name } => Some(name.clone()),
                _ => None,
            },
            None => None,
        }
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>, _symbols: &SymbolTable) {
        match t {
            Token::Register { reg_num } => {
                results.push(*reg_num);
            }
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            Token::LabelUsage { name: _ } => {}
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::*;
    use instruction::Opcode;

    #[test]
    fn test_parse_instruction_from_one() {
        let result = instruction_one(CompleteStr("hlt\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::HLT }),
                    label: None,
                    directive: None,
                    operand1: None,
                    operand2: None,
                    operand3: None,
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_from_two() {
        let result = instruction_two(CompleteStr("load $0 #100\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::LOAD }),
                    label: None,
                    directive: None,
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
                    operand3: None,
                }
            ))
        );
    }

    #[test]
    fn test_to_bytes() {
        let instruction = AssemblerInstruction {
            opcode: Some(Token::Op { code: Opcode::HLT }),
            label: None,
            directive: None,
            operand1: None,
            operand2: None,
            operand3: None,
        };
        let s = SymbolTable::new();
        let result = instruction.to_bytes(&s);
        assert_eq!(result.len(), 4)
    }
}

