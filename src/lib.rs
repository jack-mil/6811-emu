#[derive(Debug)]
pub enum AddressingMode {
    Immediate,
    Direct,
    Relative,
    IndexX,
    NoneAddressing,
}

pub mod flags {
    /*                 68HC11 status flags:                 */
    pub const NO_FLAG: u8 = 0x00; /* No flags set           */
    pub const C_FLAG:  u8 = 0x01; /* Carry occured           */
    pub const V_FLAG:  u8 = 0x02; /* Overflow occured        */
    pub const Z_FLAG:  u8 = 0x04; /* Result is zero          */
    pub const N_FLAG:  u8 = 0x08; /* Result is negative      */
    pub const I_FLAG:  u8 = 0x10; /* I interrupt mask        */
    pub const H_FLAG:  u8 = 0x20; /* Half-carry occured (from bit 3) */
    pub const X_FLAG:  u8 = 0x40; /* X interrupt mask        */
    pub const S_FLAG:  u8 = 0x80; /* Stop disable flag       */
}
use crate::flags::*;

pub struct CPU {
    pub register_a: u8,
    pub index_x: u16,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            status: 0,
            index_x: 0,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_read_u16(&self, addr: u16) -> u16 {
        let hi = self.mem_read(addr) as u16;
        let lo = self.mem_read(addr + 1) as u16;
        (hi << 8) | lo
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
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

    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::Direct => self.mem_read(self.program_counter) as u16,
            AddressingMode::Relative => {
                self.program_counter
                    .wrapping_add(self.mem_read(self.program_counter) as u16) as u16
            }
            AddressingMode::IndexX => {
                let offset = self.mem_read_u16(self.program_counter);
                self.program_counter += 1;
                self.index_x.wrapping_add(offset)
            }
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
        let operand2 = self.mem_read(addr);
        // let sum = self.register_a as u16 + operand2 as u16;
        let result = self.register_a.wrapping_add(operand2);

        // Check for carry occured
        // X7 & M7 + M7 & !R7 | X7 & !R7
        // Manual p. 496
        if ((self.register_a & operand2) | (operand2 & !result) | (!result & self.register_a)) & 0x80
            == 0
        {
            // Clear
            self.status &= !C_FLAG;
        } else {
            // Set
            self.status |= C_FLAG;
        }

        // Clear or set overflow flag if needed
        // X7 & M7 & !R7 | !X7 & !M7 & R7
        // Manual p. 496
        if ((operand2 & self.register_a & !result) | (!operand2 & !self.register_a & result)) & 0x80
            == 0
        {
            // Clear
            self.status &= !V_FLAG;
        } else {
            // Set
            self.status |= V_FLAG;
        }
        self.update_zero_and_negative_flags(result);
        self.register_a = result;
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            // Result is 0, set Z flag
            self.status |= Z_FLAG;
        } else {
            // Result is not 0, clear Z flag
            self.status &= !Z_FLAG;
        }

        if result & 0x80 == 0 {
            // Result is positive, clear N flag
            self.status &= !N_FLAG;
        } else {
            // Result is negative, set N flag
            self.status |= N_FLAG;
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
