pub struct CPU {
    pub register_a: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            status: 0,
            program_counter: 0,
        }
    }

    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
        self.status = self.status | 0b0000_0010; // Ensure overflow (V) flag to 0 (from spec)
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            // Result is 0, set Z flag
            self.status = self.status | 0b0000_0100;
        } else {
            // Result is not 0, clear Z flag
            self.status = self.status & 0b1111_1011;
        }

        if result & 0b1000_0000 != 0 {
            // Result is negative, set N flag
            self.status = self.status | 0b0000_1000;
        } else {
            // Result is positive, clear N flag
            self.status = self.status & 0b1111_0111;
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0; // Reset PC to zero for new program

        loop {
            let opcode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opcode {
                // LDA (IMM)
                0x86 => {
                    // Get next byte operand and increment PC
                    let operand = program[self.program_counter as usize];
                    self.program_counter += 1;
                    self.lda(operand);
                }

                0x00 => return,
                _ => todo!(),
            }
        }
    }
}

pub mod flags {
                                    /* 68HC11 status flags:       */
    pub const C_FLAG: u8 = 0x01;        /* 1: Carry occured           */
    pub const V_FLAG: u8 = 0x02;        /* 1: Overflow occured        */
    pub const Z_FLAG: u8 = 0x04;        /* 1: Result is zero          */
    pub const N_FLAG: u8 = 0x08;        /* 1: Result is negative      */
    pub const I_FLAG: u8 = 0x10;        /* I interrupt mask           */
    pub const H_FLAG: u8 = 0x20;        /* Half-carry occured (from bit 3) */
    pub const X_FLAG: u8 = 0x40;        /* X interrupt mask           */
    pub const S_FLAG: u8 = 0x80;        /* Stop disable flag          */
}

#[cfg(test)]
mod opcode_tests {
    use super::CPU;
    use super::flags::*;
    #[test]
    fn test_lda_imm_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0x86, 0x05, 0x00]);

        assert_eq!(cpu.register_a, 5);
        assert!(cpu.status & Z_FLAG == 0);
        assert!(cpu.status & N_FLAG == 0);
    }

    #[test]
    fn test_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0x86, 0x00, 0x00]);
        assert!(cpu.status & Z_FLAG == Z_FLAG);
    }
}
