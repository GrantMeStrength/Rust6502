
 mod memory;




fn main() {
    println!("Hello, Memmory!");

    // 64Kb of RAM for the 6502
    let mut ram = memory::MemoryArray::init();
    ram.write(0x0000, 0xff);
    println!("Value at 0x0000: {}", ram.read(0x0000));
}
