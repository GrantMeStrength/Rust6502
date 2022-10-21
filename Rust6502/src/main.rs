
 mod memory;
 mod cpu;
fn main() {
    println!("6502 Startup");

    // 64Kb of RAM for the 6502
    let mut ram = memory::MemoryArray::init();

	// Put some code in memory
    ram.write(0x0000, 0);
    println!("Value at 0x0000: {}", ram.read(0x0000));

	// 6502 Implementation
	let mut cpu6502 : cpu::CPU = cpu::CPU::init();
	cpu6502.reset();
	println!("CPU: {:?}", cpu6502);

	// Run the 6502 code
	cpu6502.execute(ram);
	cpu6502.execute(ram);
	cpu6502.execute(ram);
	cpu6502.execute(ram);

	
}
