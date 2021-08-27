#[derive(Debug)]
pub enum AddressingMode {
    Immediate,
    Direct,
    Relative,
    NoneAddressing,
}

pub mod flags {
    /* 68HC11 status flags:       */
    pub const C_FLAG: u8 = 0x01; /* 1: Carry occured           */
    pub const V_FLAG: u8 = 0x02; /* 1: Overflow occured        */
    pub const Z_FLAG: u8 = 0x04; /* 1: Result is zero          */
    pub const N_FLAG: u8 = 0x08; /* 1: Result is negative      */
    pub const I_FLAG: u8 = 0x10; /* I interrupt mask           */
    pub const H_FLAG: u8 = 0x20; /* Half-carry occured (from bit 3) */
    pub const X_FLAG: u8 = 0x40; /* X interrupt mask           */
    pub const S_FLAG: u8 = 0x80; /* Stop disable flag          */
}

// pub struct OpCode {
//     code: u8,
//     mnemonic: &'static str,
//     bytes: u8,
//     cycles: u8,
//     mode: AddressingMode,

// }
// impl OpCode {
//     fn new(code:u8, mnemonic: &'static str, bytes:u8, cycles:u8, mode: AddressingMode) -> Self {
//         OpCode {
//             code: code,
//             mnemonic: mnemonic,
//             bytes: bytes,
//             cycles: cycles,
//             mode: mode,
//         }
//     }
// }

pub struct CPU {
    pub register_a: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}


use flags::*;
impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.run()
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0xE000..(0xE000 + program.len())].copy_from_slice(&program[..]);
        self.program_counter = 0xE000;
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.status = 0;

        self.program_counter = 0xE000;
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::Direct => self.mem_read(self.program_counter) as u16,
            AddressingMode::Relative => self.program_counter.wrapping_add(self.mem_read(self.program_counter) as u16) as u16,
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
        self.status &= !V_FLAG; // Ensure overflow (V) flag to 0 (from spec)
    }

    fn add(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let result = self.register_a.wrapping_add(self.mem_read(addr));
        self.register_a = result;
        self.update_zero_and_negative_flags(result);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            // Result is 0, set Z flag
            self.status |= Z_FLAG;
        } else {
            // Result is not 0, clear Z flag
            self.status &= !Z_FLAG;
        }

        if result >> 7 == 1 {
            // Result is negative, set N flag
            self.status |= N_FLAG;
        } else {
            // Result is positive, clear N flag
            self.status &= !N_FLAG;
        }
    }

    pub fn run(&mut self) {
        loop {
            // FETCH
            let opcode = self.mem_read(self.program_counter);
            self.program_counter += 1;

            // Decode & Execute
            match opcode {
                // LDA (IMM)
                0x86 => {
                    // Get next byte operand and increment PC
                    self.lda(&AddressingMode::Immediate);
                    self.program_counter += 1;
                }
                // LDA (DIR)
                0x96 => {
                    self.lda(&AddressingMode::Direct);
                    self.program_counter += 1;
                }
                // ADD (IMM)
                0x8B => {
                    self.add(&AddressingMode::Immediate);
                    self.program_counter += 1;
                }
                // ADD (DIR)
                0x9B => {
                    self.add(&AddressingMode::Direct);
                    self.program_counter += 1;
                }
                0x00 => return,
                0x3E => return,
                _ => todo!(),
            }
        }
    }
}


#[cfg(test)]
mod opcode_tests {
    use super::flags::*;
    use super::CPU;
    #[test]
    fn test_lda_imm_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0x86, 0x05, 0x3E]);

        assert_eq!(cpu.register_a, 5);
        assert!(cpu.status & Z_FLAG == 0);
        assert!(cpu.status & N_FLAG == 0);
    }
    #[test]
    fn test_lda_direct_mode() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x20, 0x0A);
        cpu.load_and_run(vec![0x96, 0x20, 0x3E]);

        assert_eq!(cpu.register_a, 10);
        assert!(cpu.status & Z_FLAG == 0);
        assert!(cpu.status & N_FLAG == 0);
    }

    #[test]
    fn test_lda_imm_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0x86, 0x00, 0x3E]);
        assert!(cpu.status & Z_FLAG == Z_FLAG);
    }

    #[test]
    fn test_add_positive() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0x8B, 0x18, 0x3E]);
        assert_eq!(cpu.register_a, 24);
    }

    #[test]
    fn test_add_negative() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0x86, 0x0C, 0x8B, 0xE8, 0x3E]);
        assert_eq!(cpu.register_a, 0xF4);
        assert!(cpu.status & N_FLAG == N_FLAG);
    }

    #[test]
    fn test_add_to_zero() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0x86, 0x08, 0x8B, 0xF8, 0x3E]);
        assert_eq!(cpu.register_a, 0x00);
        assert!(cpu.status & Z_FLAG == Z_FLAG);
    }
}
