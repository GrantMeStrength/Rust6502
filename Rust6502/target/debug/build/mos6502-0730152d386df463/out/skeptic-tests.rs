extern crate skeptic;
#[test] fn readme_sect_usage_example_line_14() {
    let s = &format!(r####"
{}"####, r####"extern crate mos6502;
use mos6502::address::Address;
use mos6502::cpu;

fn main() {

    let zero_page_data = [56, 49];

    let program = [
    // (F)irst | (S)econd
    // .algo
        0xa5, 0x00,       // Load from F to A
    // .algo_
        0x38,             // Set carry flag
        0xe5, 0x01,       // Substract S from number in A (from F)
        0xf0, 0x07,       // Jump to .end if diff is zero
        0x30, 0x08,       // Jump to .swap if diff is negative
        0x85, 0x00,       // Load A to F
        0x4c, 0x12, 0x00, // Jump to .algo_
    // .end
        0xa5, 0x00,       // Load from S to A
        0xff, 
    // .swap
        0xa6, 0x00,       // load F to X
        0xa4, 0x01,       // load S to Y
        0x86, 0x01,       // Store X to F
        0x84, 0x00,       // Store Y to S
        0x4c, 0x10, 0x00, // Jump to .algo
    ];

    let mut cpu = cpu::CPU::new();

    cpu.memory.set_bytes(Address(0x00), &zero_page_data);
    cpu.memory.set_bytes(Address(0x10), &program);
    cpu.registers.program_counter = Address(0x10);

    cpu.run();

    assert_eq!(7, cpu.registers.accumulator);
    
}
"####);
    skeptic::rt::run_test(r#"/Users/john/.cargo/registry/src/github.com-1ecc6299db9ec823/mos6502-0.2.0"#, r#"/Users/john/Developer/Rust6502/Rust6502/target/debug/build/mos6502-0730152d386df463/out"#, r#"x86_64-apple-darwin"#, s);
}

