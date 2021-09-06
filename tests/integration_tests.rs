use emu::CPU;
use emu::flags::*;

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

#[test]
fn test_add_overflow() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0x86, 0x7f, 0x8b, 0x01, 0x3E]);
    assert_eq!(cpu.register_a, 0x80);
    assert!(cpu.status & V_FLAG == V_FLAG);
    println!("Register A: {:#04X}", cpu.register_a);
    println!("CCP: {:#04X}", cpu.status);
}