// 6502 Implementation

use crate::memory::MemoryArray;

#[derive(Debug, Copy, Clone)]
pub struct CPU {
	// Registers
	pub a: u8,
	pub x: u8,
	pub y: u8,
	pub sp: u8,
	pub pc: u16,
	pub carry_flag: bool,
	pub zero_flag: bool,
	pub overflow_flag: bool,
	pub decimal_flag: bool,
	pub interrupt_disable_flag: bool,
	pub negative_flag: bool,
	pub break_flag: bool,
	pub unused_flag: bool,
}

impl CPU {

	fn new() -> CPU {
		CPU {
			a: 0,
			x: 0,
			y: 0,
			sp: 0,
			pc: 0,
			carry_flag: false,
			zero_flag: false,
			overflow_flag: false,
			decimal_flag: false,
			interrupt_disable_flag: false,
			negative_flag: false,
			break_flag: false,
			unused_flag: false,
		}
	}

	pub fn init() -> CPU {
		let cpu6502 : CPU = CPU::new();
		cpu6502
	}


	pub fn reset(&mut self) {
		self.a = 0;
		self.x = 0;
		self.y = 0;
		self.sp = 0;
		self.pc = 0;
		self.carry_flag = false;
		self.zero_flag = false;
		self.overflow_flag = false;
		self.decimal_flag = false;
		self.interrupt_disable_flag = false;
		self.negative_flag = false;
		self.break_flag = false;
		self.unused_flag = false;
	}

	pub fn execute(&mut self, ram : MemoryArray) {
		let code = ram.read(self.pc);
		match code {
			0x00 => {
				// BRK
				println!("BRK");
				self.pc += 1;
			}
			0x01 => {
				// ORA (Indirect,X)
				self.pc += 1;
			}
			0x05 => {
				// ORA Zero Page
				self.pc += 1;
			}
			0x06 => {
				// ASL Zero Page
				self.pc += 1;
			}
			0x08 => {
				// PHP
				self.pc += 1;
			}
			0x09 => {
				// ORA Immediate
				self.pc += 1;
			}
			0x0a => {
				// ASL Accumulator
				self.pc += 1;
			}
			0x0d => {
				// ORA Absolute
				self.pc += 1;
			}
			0x0e => {
				// ASL Absolute
				self.pc += 1;
			}
			0x10 => {
				// BPL
				self.pc += 1;
			}
			0x11 => {
				// ORA (Indirect),Y
				self.pc += 1;
			}
			0x15 => {
				// ORA Zero Page,X
				self.pc += 1;
			}
			0x16 => {
				// ASL Zero Page,X
				self.pc += 1;
			}
			0x18 => {
				// CLC
				self.pc += 1;
			}
			0x19 => {
				// ORA Absolute,Y
				self.pc += 1;
			}
			0x1d => {
				// ORA Absolute,X
				self.pc += 1;
			}
			0x1e => {
				// ASL Absolute,X
				self.pc += 1;
			}
			0x20 => {
				// JSR
				self.pc += 1;
			}
			0x21 => {
				// AND (Indirect,X)
				self.pc += 1;
			}
			0x24 => {
				// BIT Zero Page
				self.pc += 1;
			}

			_ => todo!(),
			
		}
	}

}