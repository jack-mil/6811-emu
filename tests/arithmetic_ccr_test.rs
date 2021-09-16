use emu::flags::*;
mod common;

#[test]
fn test_add_unsigned() {
    // Unsigned addition
    // 0xFE + 0xF5 = 0x1F3 (carry) (negative set)
    let (reg_a, status) = common::run_program(vec![0x86, 0xFE, 0x8B, 0xF5, 0x3E]);
    assert_eq!(reg_a, 0xF3);
    assert_eq!(status & 0x0F, N_FLAG | C_FLAG);

    // Unsigned addition
    // 0xA0 + 0xC7 = 0x167 (carry) (overflow set)
    let (reg_a, status) = common::run_program(vec![0x86, 0xA0, 0x8b, 0xC7, 0x3E]);
    assert_eq!(reg_a, 0x67);
    assert_eq!(status & 0x0F, C_FLAG | V_FLAG);

    // Unsigned addition
    // 0x17 + 0x11 = 0x28 (no flags set)
    let (reg_a, status) = common::run_program(vec![0x86, 0x17, 0x8b, 0x11, 0x3E]);
    assert_eq!(reg_a, 0x28);
    assert_eq!(status & 0x0F, NO_FLAG);
}

#[test]
fn test_add_signed() {
    /* Twos Complement signed addition */

    // Signed addition
    // 0x67 + 0x6B = 0xD2 (overflow set) (negative set) (carry unset) (0x0D2 = 210)
    let (reg_a, status) = common::run_program(vec![0x86, 0x67, 0x8b, 0x6B, 0x3E]);
    assert_eq!(reg_a, 0xD2);
    assert_eq!(status & 0x0F, N_FLAG | V_FLAG);

    // Signed addition
    // 0xAC + 0xAC = 0x58 (overflow set) (carry set) (0x158 = -168)
    let (reg_a, status) = common::run_program(vec![0x86, 0xAC, 0x8b, 0xAC, 0x3E]);
    assert_eq!(reg_a, 0x58);
    assert_eq!(status & 0x0F, C_FLAG | V_FLAG);
}

