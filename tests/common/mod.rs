use emu::CPU;
pub fn run_program(program: Vec<u8>) -> (u8, u8) {
    let mut cpu = CPU::new();
    cpu.load_and_run(program);

    (cpu.register_a, cpu.status)
}