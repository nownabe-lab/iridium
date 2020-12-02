use nom::types::CompleteStr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Opcode {
    LOAD, // 0
    ADD,
    SUB,
    MUL,
    DIV,
    HLT, // 5
    JMP,
    JMPF,
    JMPB,
    EQ,
    NEQ, // 10
    GT,
    LT,
    GTE,
    LTE,
    JMPE, // 15
    // NOP,
    ALOC,
    INC,
    DEC,
    IGL,
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => return Opcode::LOAD,
            1 => return Opcode::ADD,
            2 => return Opcode::SUB,
            3 => return Opcode::MUL,
            4 => return Opcode::DIV,
            5 => return Opcode::HLT,
            6 => return Opcode::JMP,
            7 => return Opcode::JMPF,
            8 => return Opcode::JMPB,
            9 => return Opcode::EQ,
            10 => return Opcode::NEQ,
            11 => return Opcode::GT,
            12 => return Opcode::LT,
            13 => return Opcode::GTE,
            14 => return Opcode::LTE,
            15 => return Opcode::JMPE,
            17 => return Opcode::ALOC,
            18 => return Opcode::INC,
            19 => return Opcode::DEC,
            _ => return Opcode::IGL
        }
    }
}

impl From<Opcode> for u8 {
    fn from(op: Opcode) -> Self {
        match op {
            Opcode::LOAD => 0,
            Opcode::ADD => 1,
            Opcode::SUB => 2,
            Opcode::MUL => 3,
            Opcode::DIV => 4,
            Opcode::HLT => 5,
            Opcode::JMP => 6,
            Opcode::JMPF => 7,
            Opcode::JMPB => 8,
            Opcode::EQ => 9,
            Opcode::NEQ => 10,
            Opcode::GT => 11,
            Opcode::LT => 12,
            Opcode::GTE => 13,
            Opcode::LTE => 14,
            Opcode::JMPE => 15,
            Opcode::ALOC => 17,
            Opcode::INC => 18,
            Opcode::DEC => 19,
            Opcode::IGL => 100,
        }
    }
}

impl<'a> From<CompleteStr<'a>> for Opcode {
    fn from(v: CompleteStr<'a>) -> Self {
        match v {
            CompleteStr(op) => op.into(),
        }
    }
}

impl<'a> From<&'a str> for Opcode {
    fn from(v: &'a str) -> Self {
        match v {
            "load" => Opcode::LOAD,
            "add" => Opcode::ADD,
            "sub" => return Opcode::SUB,
            "mul" => return Opcode::MUL,
            "div" => return Opcode::DIV,
            "hlt" => return Opcode::HLT,
            "jmp" => return Opcode::JMP,
            "jmpf" => return Opcode::JMPF,
            "jmpb" => return Opcode::JMPB,
            "eq" => return Opcode::EQ,
            "neq" => return Opcode::NEQ,
            "gt" => return Opcode::GT,
            "lt" => return Opcode::LT,
            "gte" => return Opcode::GTE,
            "lte" => return Opcode::LTE,
            "jmpe" => return Opcode::JMPE,
            "aloc" => return Opcode::ALOC,
            "inc" => return Opcode::INC,
            "dec" => return Opcode::DEC,
            _ => return Opcode::IGL
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction {
            opcode: opcode
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }

    #[test]
    fn test_str_to_opcode() {
        let opcode = Opcode::from(CompleteStr("load"));
        assert_eq!(opcode, Opcode::LOAD);
        let opcode = Opcode::from(CompleteStr("illegal"));
        assert_eq!(opcode, Opcode::IGL);
    }
}
