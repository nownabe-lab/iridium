use instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
    equal_flag: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
            equal_flag: false,
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    pub fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32;
            },
            Opcode::ADD => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = val1 + val2;
            },
            Opcode::SUB => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = val1 - val2;
            },
            Opcode::MUL => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = val1 * val2;
            },
            Opcode::DIV => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = val1 / val2;
                self.remainder = (val1 % val2) as u32;
            },
            Opcode::HLT => {
                println!("HLT encountered");
                return false;
            },
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            },
            Opcode::JMPF => {
                let val = self.registers[self.next_8_bits() as usize];
                self.pc += val as usize;
            },
            Opcode::JMPB => {
                let val = self.registers[self.next_8_bits() as usize];
                self.pc -= val as usize;
            },
            Opcode::EQ => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = val1 == val2;
                self.next_8_bits();
            },
            Opcode::NEQ => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = val1 != val2;
                self.next_8_bits();
            },
            Opcode::GT => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = val1 > val2;
                self.next_8_bits();
            },
            Opcode::LT => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = val1 < val2;
                self.next_8_bits();
            },
            Opcode::GTE => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = val1 >= val2;
                self.next_8_bits();
            },
            Opcode::LTE => {
                let val1 = self.registers[self.next_8_bits() as usize];
                let val2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = val1 <= val2;
                self.next_8_bits();
            },
            Opcode::JMPE => {
                if self.equal_flag {
                    let target = self.registers[self.next_8_bits() as usize];
                    self.pc = target as usize;
                } else {
                    self.next_8_bits();
                    self.next_16_bits();
                }
            },
            Opcode::IGL => {
                println!("Unrecognized opcode found! Terminating!");
                return false;
            }
        }
        true
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_add_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 2];
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 10;
        test_vm.run();
        assert_eq!(test_vm.registers[2], 15);
    }

    #[test]
    fn test_sub_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![2, 0, 1, 2];
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 5;
        test_vm.run();
        assert_eq!(test_vm.registers[2], 5);
    }

    #[test]
    fn test_mul_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![3, 0, 1, 2];
        test_vm.registers[0] = 2;
        test_vm.registers[1] = 3;
        test_vm.run();
        assert_eq!(test_vm.registers[2], 6);
    }

    #[test]
    fn test_div_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![4, 0, 1, 2];
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 3;
        test_vm.run();
        assert_eq!(test_vm.registers[2], 3);
        assert_eq!(test_vm.remainder, 1);
    }

    #[test]
    fn test_hlt_opcode() {
        let mut test_vm = VM::new();
        let test_bytes = vec![5,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.registers[0] = 3;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 3);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut vm = VM::new();
        vm.program = vec![7, 0, 0, 0];
        vm.registers[0] = 2;
        vm.run_once();
        assert_eq!(vm.pc, 4);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut vm = VM::new();
        vm.program = vec![8, 0, 0, 0];
        vm.registers[0] = 2;
        vm.run_once();
        assert_eq!(vm.pc, 0);
    }

    #[test]
    fn test_eq_opcode() {
        let mut vm = VM::new();
        vm.program = vec![9, 0, 1, 0];
        vm.registers[0] = 10;
        vm.registers[1] = 10;
        vm.run_once();
        assert_eq!(vm.equal_flag, true);
        assert_eq!(vm.pc, 4);

        let mut vm = VM::new();
        vm.program = vec![9, 0, 1, 0];
        vm.registers[0] = 10;
        vm.registers[1] = 20;
        vm.run_once();
        assert_eq!(vm.equal_flag, false);
        assert_eq!(vm.pc, 4);
    }

    #[test]
    fn test_neq_opcode() {
        let mut vm = VM::new();
        vm.program = vec![10, 0, 1, 0];
        vm.registers[0] = 10;
        vm.registers[1] = 10;
        vm.run_once();
        assert_eq!(vm.equal_flag, false);
        assert_eq!(vm.pc, 4);

        let mut vm = VM::new();
        vm.program = vec![10, 0, 1, 0];
        vm.registers[0] = 10;
        vm.registers[1] = 20;
        vm.run_once();
        assert_eq!(vm.equal_flag, true);
        assert_eq!(vm.pc, 4);
    }

    #[test]
    fn test_gt_opcode() {
        let mut vm = VM::new();
        vm.program = vec![11, 0, 1, 0];
        vm.registers[0] = 10;
        vm.registers[1] = 20;
        vm.run_once();
        assert_eq!(vm.equal_flag, false);
        assert_eq!(vm.pc, 4);

        let mut vm = VM::new();
        vm.program = vec![11, 0, 1, 0];
        vm.registers[0] = 20;
        vm.registers[1] = 10;
        vm.run_once();
        assert_eq!(vm.equal_flag, true);
        assert_eq!(vm.pc, 4);

        let mut vm = VM::new();
        vm.program = vec![11, 0, 1, 0];
        vm.registers[0] = 10;
        vm.registers[1] = 10;
        vm.run_once();
        assert_eq!(vm.equal_flag, false);
        assert_eq!(vm.pc, 4);
    }

    #[test]
    fn test_lt_opcode() {
        let op = 12;

        let mut vm = VM::new();
        vm.program = vec![op, 0, 1, 0];
        vm.registers[0] = 10;
        vm.registers[1] = 20;
        vm.run_once();
        assert_eq!(vm.equal_flag, true);
        assert_eq!(vm.pc, 4);

        let mut vm = VM::new();
        vm.program = vec![op, 0, 1, 0];
        vm.registers[0] = 20;
        vm.registers[1] = 10;
        vm.run_once();
        assert_eq!(vm.equal_flag, false);
        assert_eq!(vm.pc, 4);

        let mut vm = VM::new();
        vm.program = vec![op, 0, 1, 0];
        vm.registers[0] = 10;
        vm.registers[1] = 10;
        vm.run_once();
        assert_eq!(vm.equal_flag, false);
        assert_eq!(vm.pc, 4);
    }

    #[test]
    fn test_gte_opcode() {
        let mut vm = VM::new();
        vm.program = vec![13, 0, 1, 0];
        vm.registers[0] = 20;
        vm.registers[1] = 10;
        vm.run_once();
        assert_eq!(vm.equal_flag, true);
        assert_eq!(vm.pc, 4);

        let mut vm = VM::new();
        vm.program = vec![13, 0, 1, 0];
        vm.registers[0] = 10;
        vm.registers[1] = 20;
        vm.run_once();
        assert_eq!(vm.equal_flag, false);
        assert_eq!(vm.pc, 4);

        let mut vm = VM::new();
        vm.program = vec![13, 0, 1, 0];
        vm.registers[0] = 10;
        vm.registers[1] = 10;
        vm.run_once();
        assert_eq!(vm.equal_flag, true);
        assert_eq!(vm.pc, 4);
    }

    #[test]
    fn test_lte_opcode() {
        let mut vm = VM::new();
        vm.program = vec![14, 0, 1, 0];
        vm.registers[0] = 10;
        vm.registers[1] = 20;
        vm.run_once();
        assert_eq!(vm.equal_flag, true);
        assert_eq!(vm.pc, 4);

        let mut vm = VM::new();
        vm.program = vec![14, 0, 1, 0];
        vm.registers[0] = 20;
        vm.registers[1] = 10;
        vm.run_once();
        assert_eq!(vm.equal_flag, false);
        assert_eq!(vm.pc, 4);

        let mut vm = VM::new();
        vm.program = vec![14, 0, 1, 0];
        vm.registers[0] = 10;
        vm.registers[1] = 10;
        vm.run_once();
        assert_eq!(vm.equal_flag, true);
        assert_eq!(vm.pc, 4);
    }

    #[test]
    fn test_jmpe_opcode() {
        let op = 15;

        let mut vm = VM::new();
        vm.program = vec![op, 0, 0, 0];
        vm.registers[0] = 0;
        vm.equal_flag = true;
        vm.run_once();
        assert_eq!(vm.pc, 0);

        let mut vm = VM::new();
        vm.program = vec![op, 0, 0, 0];
        vm.registers[0] = 0;
        vm.run_once();
        assert_eq!(vm.pc, 4);
    }

    #[test]
    fn test_igl_opcode() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }
}

