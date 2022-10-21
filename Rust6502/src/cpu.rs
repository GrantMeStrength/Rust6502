// 
// 6502 Implementation
//

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
	pub interrupt_flag: bool,
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
			interrupt_flag: false,
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
		self.interrupt_flag = false;
		self.negative_flag = false;
		self.break_flag = false;
		self.unused_flag = false;
	}

	pub fn execute(&mut self, ram : MemoryArray) {
		let code: u8 = ram.read(self.pc);
		match code {
			0x00 => {
				self.brk(ram);
			}
			0x01 => {
				self.ora_indirect_x(ram );
			}
			0x05 => {
				self.ora_zero_page(ram);
			}
			0x06 => {
				self.asl_zero_page(ram);
			}
			0x08 => {
				self.php(ram);
			}
			0x09 => {
				self.ora_immediate(ram);
			}
			0x0a => {
				self.asl_accumulator();
			}
			0x0d => {
				self.ora_absolute(ram);
			}
			0x0e => {
				self.asl_absolute(ram);
			}
			0x10 => {
				self.bpl(ram);
			}
			0x11 => {
				self.ora_indirect(ram);
			}
			0x15 => {
				self.ora_zero_page_x(ram);
			}
			0x16 => {
				self.asl_zero_page_x(ram);
			}
			0x18 => {
				self.clc();
			}
			0x19 => {
				self.ora_absolute_y(ram);
			}

			0x1d => {
				self.ora_absolute_x(ram);
			}
			0x1e => {
				self.asl_absolute_x(ram);
			}
			0x20 => {
				self.jsr(ram);
			}
			0x21 => {
				self.and_indirect_x(ram);
			}
			0x24 => {
				self.bit_zero_page(ram);
			}

			0x25 => {
				self.and_zero_page(ram);
			}

			0x26 => {
				self.rol_zero_page(ram);
			}

			0x28 => {
				self.plp(ram);
			}

			0x29 => {
				self.and_immediate(ram);
			}

			0x2a => {
				self.rol_accumulator();
			}

			0x2c => {
				self.bit_absolute(ram);
			}

			0x2d => {
				self.and_absolute(ram);
			}

			0x2e => {
				self.rol_absolute(ram);
			}

			0x30 => {
				self.bmi(ram);
			}

			0x31 => {
				self.and_indirect_y(ram);
			}

			0x35 => {
				self.and_zero_page_x(ram);
			}

			0x36 => {
				self.rol_zero_page_x(ram);
			}

			0x38 => {
				self.sec();
			}

			0x39 => {
				self.and_absolute_y(ram);
			}

			0x3d => {
				self.and_absolute_x(ram);
			}

			0x3e => {
				self.rol_absolute_x(ram);
			}

			0x40 => {
				self.rti(ram);
			}
			
			
			0x41 => {
				self.eor_indirect_x(ram);
			}

			0x45 => {
				self.eor_zero_page(ram);
			}

			0x46 => {
				self.lsr_zero_page(ram);
			}

			0x48 => {
				self.pha(ram);
			}

			0x49 => {
				self.eor_immediate(ram);
			}

			0x4a => {
				self.lsr_accumulator();
			}

			0x4c => {
				self.jmp_absolute(ram);
			}

			0x4d => {
				self.eor_absolute(ram);
			}

			0x4e => {
				self.lsr_absolute(ram);
			}

			0x50 => {
				self.bvc(ram);
			}

			0x51 => {
				self.eor_indirect_y(ram);
			}

			0x55 => {
				self.eor_zero_page_x(ram);
			}

			0x56 => {
				self.lsr_zero_page_x(ram);
			}

			0x58 => {
				self.cli();
			}

			0x59 => {
				self.eor_absolute_y(ram);
			}

			0x5d => {
				self.eor_absolute_x(ram);
			}

			0x5e => {
				self.lsr_absolute_x(ram);
			}

			0x60 => {
				self.rts(ram);
			}

			0x61 => {
				self.adc_indirect_x(ram);
			}

			0x65 => {
				self.adc_zero_page(ram);
			}

			0x66 => {
				self.ror_zero_page(ram);
			}

			0x68 => {
				self.pla(ram);
			}

			0x69 => {
				self.adc_immediate(ram);
			}

			0x6a => {
				self.ror_accumulator();
			}

			0x6c => {
				self.jmp_indirect(ram);
			}

			0x6d => {
				self.adc_absolute(ram);
			}

			0x6e => {
				self.ror_absolute(ram);
			}

			0x70 => {
				self.bvs(ram);
			}

			0x71 => {
				self.adc_indirect_y(ram);
			}

			0x75 => {
				self.adc_zero_page_x(ram);
			}

			0x76 => {
				self.ror_zero_page_x(ram);
			}

			0x78 => {
				self.sei();
			}

			0x79 => {
				self.adc_absolute_y(ram);
			}

			0x7d => {
				self.adc_absolute_x(ram);
			}

			0x7e => {
				self.ror_absolute_x(ram);
			}

			0x81 => {
				self.sta_indirect_x(ram);
			}

			0x84 => {
				self.sty_zero_page(ram);
			}

			0x85 => {
				self.sta_zero_page(ram);
			}

			0x86 => {
				self.stx_zero_page(ram);
			}

			0x88 => {
				self.dey();
			}

			0x8a => {
				self.txa();
			}

			0x8c => {
				self.sty_absolute(ram);
			}

			0x8d => {
				self.sta_absolute(ram);
			}

			0x8e => {
				self.stx_absolute(ram);
			}

			0x90 => {
				self.bcc(ram);
			}

			0x91 => {
				self.sta_indirect_y(ram);
			}

			0x94 => {
				self.sty_zero_page_x(ram);
			}

			0x95 => {
				self.sta_zero_page_x(ram);
			}

			0x96 => {
				self.stx_zero_page_y(ram);
			}

			0x98 => {
				self.tya();
			}

			0x99 => {
				self.sta_absolute_y(ram);
			}

			0x9a => {
				self.txs();
			}

			0x9d => {
				self.sta_absolute_x(ram);
			}

			0xa0 => {
				self.ldy_immediate(ram);
			}

			0xa1 => {
				self.lda_indirect_x(ram);
			}

			0xa2 => {
				self.ldx_immediate(ram);
			}

			0xa4 => {
				self.ldy_zero_page(ram);
			}

			0xa5 => {
				self.lda_zero_page(ram);
			}

			0xa6 => {
				self.ldx_zero_page(ram);
			}

			0xa8 => {
				self.tay();
			}

			0xa9 => {
				self.lda_immediate(ram);
			}

			0xaa => {
				self.tax();
			}

			0xac => {
				self.ldy_absolute(ram);
			}

			0xad => {
				self.lda_absolute(ram);
			}

			0xae => {
				self.ldx_absolute(ram);
			}

			0xb0 => {
				self.bcs(ram);
			}

			0xb1 => {
				self.lda_indirect_y(ram);
			}

			0xb4 => {
				self.ldy_zero_page_x(ram);
			}

			0xb5 => {
				self.lda_zero_page_x(ram);
			}

			0xb6 => {
				self.ldx_zero_page_y(ram);
			}

			0xb8 => {
				self.clv();
			}

			0xb9 => {
				self.lda_absolute_y(ram);
			}

			0xba => {
				self.tsx();
			}

			0xbc => {
				self.ldy_absolute_x(ram);
			}

			0xbd => {
				self.lda_absolute_x(ram);
			}

			0xbe => {
				self.ldx_absolute_y(ram);
			}

			0xc0 => {
				self.cpy_immediate(ram);
			}

			0xc1 => {
				self.cmp_indirect_x(ram);
			}

			0xc4 => {
				self.cpy_zero_page(ram);
			}

			0xc5 => {
				self.cmp_zero_page(ram);
			}

			0xc6 => {
				self.dec_zero_page(ram);
			}

			0xc8 => {
				self.iny();
			}

			0xc9 => {
				self.cmp_immediate(ram);
			}

			0xca => {
				self.dex();
			}

			0xcc => {
				self.cpy_absolute(ram);
			}

			0xcd => {
				self.cmp_absolute(ram);
			}

			0xce => {
				self.dec_absolute(ram);
			}

			0xd0 => {
				self.bne(ram);
			}

			0xd1 => {
				self.cmp_indirect_y(ram);
			}

			0xd5 => {
				self.cmp_zero_page_x(ram);
			}

			0xd6 => {
				self.dec_zero_page_x(ram);
			}

			0xd8 => {
				self.cld();
			}

			0xd9 => {
				self.cmp_absolute_y(ram);
			}

			0xdd => {
				self.cmp_absolute_x(ram);
			}

			0xde => {
				self.dec_absolute_x(ram);
			}

			0xe0 => {
				self.cpx_immediate(ram);
			}

			0xe1 => {
				self.sbc_indirect_x(ram);
			}

			0xe4 => {
				self.cpx_zero_page(ram);
			}

			0xe5 => {
				self.sbc_zero_page(ram);
			}

			0xe6 => {
				self.inc_zero_page(ram);
			}

			0xe8 => {
				self.inx();
			}

			0xe9 => {
				self.sbc_immediate(ram);
			}

			0xea => {
				self.nop();
			}

			0xec => {
				self.cpx_absolute(ram);
			}

			0xed => {
				self.sbc_absolute(ram);
			}

			0xee => {
				self.inc_absolute(ram);
			}

			0xf0 => {
				self.beq(ram);
			}

			0xf1 => {
				self.sbc_indirect_y(ram);
			}

			0xf5 => {
				self.sbc_zero_page_x(ram);
			}

			0xf6 => {
				self.inc_zero_page_x(ram);
			}

			0xf8 => {
				self.sed();
			}

			0xf9 => {
				self.sbc_absolute_y(ram);
			}

			0xfd => {
				self.sbc_absolute_x(ram);
			}

			0xfe => {
				self.inc_absolute_x(ram);
			}

			_ => {
				panic!("Invalid opcode: {:x}", code);
			}




			_ => todo!(),
			
		}
	}

	
	fn compare(&mut self, a: u8, b: u8) {
		let result = a.wrapping_sub(b);
		if result == 0 {self.zero_flag = true;} else {self.zero_flag = false;}
		if a >= b {self.carry_flag = true;} else {self.carry_flag = false;}
		if (a & 0x80) != (b & 0x80) && (a & 0x80) != (result & 0x80) {self.overflow_flag = true;} else {self.overflow_flag = false;}
		if result & 0x80 == 0x80 {self.negative_flag = true;} else {self.negative_flag = false;}
	}
	
	fn set_sr(&mut self, value: u8) {
		self.carry_flag = value & 0x01 != 0;
		self.zero_flag = value & 0x02 != 0;
		self.interrupt_flag = value & 0x04 != 0;
		self.decimal_flag = value & 0x08 != 0;
		self.break_flag = value & 0x10 != 0;
		self.overflow_flag = value & 0x40 != 0;
		self.negative_flag = value & 0x80 != 0;
	}

	fn set_flags(&mut self, value : u8) {
		self.zero_flag = value == 0;
		self.negative_flag = value & 0x80 != 0;
	}

	fn get_address_at_address(&self, ram : MemoryArray, address : u16) -> u16 {
		let low_byte = ram.read(address) as u16;
		let high_byte = ram.read(address + 1) as u16;
		let address = (high_byte << 8) | low_byte;
		address
	}

	fn get_status_register(&self) -> u8 {
		let mut sr : u8 = 0;
		if self.carry_flag { sr |= 0b00000001; }
		if self.zero_flag { sr |= 0b00000010; }
		if self.interrupt_flag { sr |= 0b00000100; }
		if self.decimal_flag { sr |= 0b00001000; }
		if self.break_flag { sr |= 0b00010000; }
		if self.overflow_flag { sr |= 0b01000000; }
		if self.negative_flag { sr |= 0b10000000; }
		sr
	}

	fn push_stack(&mut self, mut ram : MemoryArray, value : u8) {
		ram.write( 0x0100 + self.sp as u16, value);
		self.sp -= 1;
	}

	fn pop_stack(&mut self, ram : MemoryArray) -> u8 {
		self.sp += 1;
		ram.read(0x0100 + self.sp as u16)
	}


	fn get_indexed_indirect_zeropage_x_address(&mut self, ram: MemoryArray) -> u16 {
		let address = self.pc; 
		self.pc = self.pc.wrapping_add(1);
		let offset : u16 = address.wrapping_add(self.x as u16) ;
		let low_byte = ram.read(offset) as u16;
		let high_byte = ram.read(offset + 1) as u16;
		let address = (high_byte << 8) | low_byte;
		address
	}

	// 6502 Instruction Set

	fn brk(&mut self, ram : MemoryArray) {
		let h: u8 = (self.pc >> 8) as u8; self.push_stack(ram, h);
		let l: u8 = (self.pc & 0xff) as u8; self.push_stack(ram, l);
		let sr : u8 = self.get_status_register(); self.push_stack(ram, sr);
		self.pc = self.get_address_at_address(ram, 0x17fa);
	}

	fn ora_indirect_x(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value : u8 = ram.read(address);
		self.a |= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn ora_zero_page(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		self.a |= value;
		self.set_flags(self.a);
		self.pc += 1;
	}	

	fn asl_zero_page(&mut self, mut ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		let result: u8 = value << 1;
		self.set_flags(result);
		ram.write(address, result);
		self.pc += 1;
	}

	fn php(&mut self, ram : MemoryArray) {
		let sr: u8 = self.get_status_register();
		self.push_stack(ram, sr);
		self.pc += 1;
	}

	fn ora_immediate(&mut self, ram : MemoryArray) {
		let value: u8 = ram.read(self.pc);
		self.a |= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn asl_accumulator(&mut self) {
		let value: u8 = self.a;
		let result: u8 = value << 1;
		self.set_flags(result);
		self.a = result;
		self.pc += 1;
	}

	fn ora_absolute(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a |= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn asl_absolute(&mut self, mut ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		let result: u8 = value << 1;
		self.set_flags(result);
		ram.write(address, result);
		self.pc += 1;
	}

	fn bpl(&mut self, ram : MemoryArray) {
		let value: u8 = ram.read(self.pc);
		if !self.negative_flag {
			self.pc += value as u16;
		}
		self.pc += 1;
	}

	fn ora_indirect(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a |= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn ora_zero_page_x(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		self.a |= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn asl_zero_page_x(&mut self, mut ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		let result: u8 = value << 1;
		self.set_flags(result);
		ram.write(address, result);
		self.pc += 1;
	}

	fn clc(&mut self) {
		self.carry_flag = false;
		self.pc += 1;
	}

	fn ora_absolute_y(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a |= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn ora_absolute_x(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a |= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn asl_absolute_x(&mut self, mut ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		let result: u8 = value << 1;
		self.set_flags(result);
		ram.write(address, result);
		self.pc += 1;
	}

	fn jsr(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let h: u8 = (self.pc >> 8) as u8; self.push_stack(ram, h);
		let l: u8 = (self.pc & 0xff) as u8; self.push_stack(ram, l);
		self.pc = address;
	}

	fn and_indirect_x(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a &= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn bit_zero_page(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		self.set_flags(value);
		self.pc += 1;
	}

	fn and_zero_page(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		self.a &= value;
		self.set_flags(self.a);
		self.pc += 1;
	}


	fn rol_zero_page(&mut self, mut ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		let result: u8 = value << 1;
		self.set_flags(result);
		ram.write(address, result);
		self.pc += 1;
	}

	fn plp(&mut self, ram : MemoryArray) {
		let sr: u8 = self.pop_stack(ram);
		self.set_sr(sr);
		self.pc += 1;
	}

	fn and_immediate(&mut self, ram : MemoryArray) {
		let value: u8 = ram.read(self.pc);
		self.a &= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn rol_accumulator(&mut self) {
		let value: u8 = self.a;
		let result: u8 = value << 1;
		self.set_flags(result);
		self.a = result;
		self.pc += 1;
	}

	fn bit_absolute(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.set_flags(value);
		self.pc += 1;
	}

	fn and_absolute(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a &= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn rol_absolute(&mut self, mut ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		let result: u8 = value << 1;
		self.set_flags(result);
		ram.write(address, result);
		self.pc += 1;
	}

	fn bmi(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		if self.negative_flag {
			self.pc = address;
		} else {
			self.pc += 1;
		}
	}

	fn and_indirect_y(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a &= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn and_zero_page_x(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		self.a &= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn rol_zero_page_x(&mut self, mut ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		let result: u8 = value << 1;
		self.set_flags(result);
		ram.write(address, result);
		self.pc += 1;
	}

	fn sec(&mut self) {
		self.carry_flag = true;
		self.pc += 1;
	}

	fn and_absolute_y(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a &= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn and_absolute_x(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a &= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn rol_absolute_x(&mut self, mut ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		let result: u8 = value << 1;
		self.set_flags(result);
		ram.write(address, result);
		self.pc += 1;
	}

	fn rti(&mut self, ram : MemoryArray) {
		let sr: u8 = self.pop_stack(ram);
		self.set_sr(sr);
		let address: u16 = self.pop_stack(ram) as u16;
		self.pc = address;
	}

	fn eor_indirect_x(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a ^= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn eor_zero_page(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		self.a ^= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn lsr_zero_page(&mut self, mut ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		let result: u8 = value >> 1;
		self.set_flags(result);
		ram.write(address, result);
		self.pc += 1;
	}

	fn pha(&mut self,  ram : MemoryArray) {
		self.push_stack(ram, self.a);
		self.pc += 1;
	}

	fn eor_immediate(&mut self, ram : MemoryArray) {
		let value: u8 = ram.read(self.pc);
		self.a ^= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn lsr_accumulator(&mut self) {
		let value: u8 = self.a;
		let result: u8 = value >> 1;
		self.set_flags(result);
		self.a = result;
		self.pc += 1;
	}

	fn jmp_absolute(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		self.pc = address;
	}

	fn eor_absolute(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a ^= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn lsr_absolute(&mut self, mut ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		let result: u8 = value >> 1;
		self.set_flags(result);
		ram.write(address, result);
		self.pc += 1;
	}

	fn bvc(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		if !self.overflow_flag {
			self.pc = address;
		} else {
			self.pc += 1;
		}
	}

	fn eor_indirect_y(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a ^= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn eor_zero_page_x(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		self.a ^= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn lsr_zero_page_x(&mut self, mut ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		let result: u8 = value >> 1;
		self.set_flags(result);
		ram.write(address, result);
		self.pc += 1;
	}

	fn cli(&mut self) {
		self.interrupt_flag = false;
		self.pc += 1;
	}

	fn eor_absolute_y(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a ^= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn eor_absolute_x(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a ^= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn lsr_absolute_x(&mut self, mut ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		let result: u8 = value >> 1;
		self.set_flags(result);
		ram.write(address, result);
		self.pc += 1;
	}

	fn rts(&mut self, ram : MemoryArray) {
		let address: u16 = self.pop_stack(ram) as u16;
		self.pc = address;
	}

	fn adc_indirect_x(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a += value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn adc_zero_page(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		self.a += value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn ror_zero_page(&mut self, mut ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		let result: u8 = value >> 1;
		self.set_flags(result);
		ram.write(address, result);
		self.pc += 1;
	}

	fn pla(&mut self, ram : MemoryArray) {
		self.a = self.pop_stack(ram);
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn adc_immediate(&mut self, ram : MemoryArray) {
		let value: u8 = ram.read(self.pc);
		self.a += value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn ror_accumulator(&mut self) {
		let value: u8 = self.a;
		let result: u8 = value >> 1;
		self.set_flags(result);
		self.a = result;
		self.pc += 1;
	}

	fn jmp_indirect(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		self.pc = address;
	}

	fn adc_absolute(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a += value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn ror_absolute(&mut self, mut ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		let result: u8 = value >> 1;
		self.set_flags(result);
		ram.write(address, result);
		self.pc += 1;
	}

	fn bvs(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		if self.overflow_flag {
			self.pc = address;
		} else {
			self.pc += 1;
		}
	}

	fn adc_indirect_y(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a += value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn adc_zero_page_x(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		self.a += value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn ror_zero_page_x(&mut self, mut ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		let result: u8 = value >> 1;
		self.set_flags(result);
		ram.write(address, result);
		self.pc += 1;
	}

	fn sei(&mut self) {
		self.interrupt_flag = true;
		self.pc += 1;
	}

	fn adc_absolute_y(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a += value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn adc_absolute_x(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a += value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn ror_absolute_x(&mut self, mut ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		let result: u8 = value >> 1;
		self.set_flags(result);
		ram.write(address, result);
		self.pc += 1;
	}

	fn sta_indirect_x(&mut self, mut ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		ram.write(address, self.a);
		self.pc += 1;
	}

	fn sty_zero_page(&mut self, mut ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		ram.write(address, self.y);
		self.pc += 1;
	}

	fn sta_zero_page(&mut self, mut ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		ram.write(address, self.a);
		self.pc += 1;
	}

	fn stx_zero_page(&mut self, mut ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		ram.write(address, self.x);
		self.pc += 1;
	}

	fn dey(&mut self) {
		self.y -= 1;
		self.set_flags(self.y);
		self.pc += 1;
	}

	fn txa(&mut self) {
		self.a = self.x;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn tsx(&mut self) {
		self.x = self.sp;
		self.set_flags(self.x);
		self.pc += 1;
	}

	fn lda_indirect_y(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		self.a = ram.read(address);
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn lda_absolute_y(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		self.a = ram.read(address);
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn sty_absolute(&mut self, mut ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		ram.write(address, self.y);
		self.pc += 1;
	}

	fn txs(&mut self) {
		self.sp = self.x;
		self.pc += 1;
	}

	fn cmp_indirect_x(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.compare(self.a, value);
		self.pc += 1;
	}

	fn dex(&mut self) {
		self.x -= 1;
		self.set_flags(self.x);
		self.pc += 1;
	}

	fn bne(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		if !self.zero_flag {
			self.pc = address;
		} else {
			self.pc += 1;
		}
	}

	fn cld(&mut self) {
		self.decimal_flag = false;
		self.pc += 1;
	}

	fn cmp_absolute_y(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.compare(self.a, value);
		self.pc += 1;
	}

	fn nop(&mut self) {
		self.pc += 1;
	}

	fn cpx_absolute(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.compare(self.x, value);
		self.pc += 1;
	}

	fn inc_absolute(&mut self, mut ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		let result: u8 = value + 1;
		self.set_flags(result);
		ram.write(address, result);
		self.pc += 1;
	}

	fn beq(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		if self.zero_flag {
			self.pc = address;
		} else {
			self.pc += 1;
		}
	}

	fn inc_zero_page_x(&mut self, mut ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		let result: u8 = value + 1;
		self.set_flags(result);
		ram.write(address, result);
		self.pc += 1;
	}

	fn inc_absolute_x(&mut self, mut ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		let result: u8 = value + 1;
		self.set_flags(result);
		ram.write(address, result);
		self.pc += 1;
	}



	fn sta_absolute(&mut self, mut ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		ram.write(address, self.a);
		self.pc += 1;
	}

	fn stx_absolute(&mut self, mut ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		ram.write(address, self.x);
		self.pc += 1;
	}

	fn bcc(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		if !self.carry_flag {
			self.pc = address;
		} else {
			self.pc += 1;
		}
	}

	fn sty_zero_page_x(&mut self, mut ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		ram.write(address, self.y);
		self.pc += 1;
	}

	fn sta_zero_page_x(&mut self, mut ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		ram.write(address, self.a);
		self.pc += 1;
	}

	fn stx_zero_page_y(&mut self, mut ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		ram.write(address, self.x);
		self.pc += 1;
	}

	fn tya(&mut self) {
		self.a = self.y;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn sta_absolute_y(&mut self, mut ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		ram.write(address, self.a);
		self.pc += 1;
	}

	fn sta_absolute_x(&mut self, mut ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		ram.write(address, self.a);
		self.pc += 1;
	}

	fn ldy_immediate(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		self.y = ram.read(address);
		self.set_flags(self.y);
		self.pc += 1;
	}

	fn lda_indirect_x(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		self.a = ram.read(address);
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn ldx_immediate(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		self.x = ram.read(address);
		self.set_flags(self.x);
		self.pc += 1;
	}

	fn ldy_zero_page(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		self.y = ram.read(address);
		self.set_flags(self.y);
		self.pc += 1;
	}

	fn lda_zero_page(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		self.a = ram.read(address);
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn ldx_zero_page(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		self.x = ram.read(address);
		self.set_flags(self.x);
		self.pc += 1;
	}

	fn tay(&mut self) {
		self.y = self.a;
		self.set_flags(self.y);
		self.pc += 1;
	}

	fn lda_immediate(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		self.a = ram.read(address);
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn tax(&mut self) {
		self.x = self.a;
		self.set_flags(self.x);
		self.pc += 1;
	}

	fn ldy_absolute(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		self.y = ram.read(address);
		self.set_flags(self.y);
		self.pc += 1;
	}

	fn lda_absolute(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		self.a = ram.read(address);
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn ldx_absolute(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		self.x = ram.read(address);
		self.set_flags(self.x);
		self.pc += 1;
	}

	fn bcs(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		if self.carry_flag {
			self.pc = address;
		} else {
			self.pc += 1;
		}
	}

	fn ldy_zero_page_x(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		self.y = ram.read(address);
		self.set_flags(self.y);
		self.pc += 1;
	}

	fn lda_zero_page_x(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		self.a = ram.read(address);
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn ldx_zero_page_y(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		self.x = ram.read(address);
		self.set_flags(self.x);
		self.pc += 1;
	}

	fn clv(&mut self) {
		self.overflow_flag = false;
		self.pc += 1;
	}

	fn ldy_absolute_x(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		self.y = ram.read(address);
		self.set_flags(self.y);
		self.pc += 1;
	}

	fn lda_absolute_x(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		self.a = ram.read(address);
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn ldx_absolute_y(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		self.x = ram.read(address);
		self.set_flags(self.x);
		self.pc += 1;
	}

	fn cpy_immediate(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.set_flags(self.y - value);
		self.carry_flag = self.y >= value;
		self.pc += 1;
	}

	fn cmp_indirect_y(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.set_flags(self.a - value);
		self.carry_flag = self.a >= value;
		self.pc += 1;
	}

	fn cpy_zero_page(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		self.set_flags(self.y - value);
		self.carry_flag = self.y >= value;
		self.pc += 1;
	}

	fn cmp_zero_page(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		self.set_flags(self.a - value);
		self.carry_flag = self.a >= value;
		self.pc += 1;
	}

	fn dec_zero_page(&mut self, mut ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		ram.write(address, value - 1);
		self.set_flags(value - 1);
		self.pc += 1;
	}

	fn iny(&mut self) {
		self.y += 1;
		self.set_flags(self.y);
		self.pc += 1;
	}

	fn cpx_immediate(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.set_flags(self.x - value);
		self.carry_flag = self.x >= value;
		self.pc += 1;
	}

	fn sbc_immediate(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a -= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn cpx_zero_page(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		self.set_flags(self.x - value);
		self.carry_flag = self.x >= value;
		self.pc += 1;
	}

	fn sbc_zero_page(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		self.a -= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn inc_zero_page(&mut self, mut ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		ram.write(address, value + 1);
		self.set_flags(value + 1);
		self.pc += 1;
	}

	fn inx(&mut self) {
		self.x += 1;
		self.set_flags(self.x);
		self.pc += 1;
	}

	fn cpy_absolute(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.set_flags(self.y - value);
		self.carry_flag = self.y >= value;
		self.pc += 1;
	}

	fn cmp_absolute(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.set_flags(self.a - value);
		self.carry_flag = self.a >= value;
		self.pc += 1;
	}

	fn dec_absolute(&mut self, mut ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		ram.write(address, value - 1);
		self.set_flags(value - 1);
		self.pc += 1;
	}

	// fn cpx_absolute_y(&mut self, ram : MemoryArray) {
	// 	let address: u16 = self.get_address_at_address(ram, self.pc);
	// 	let value: u8 = ram.read(address);
	// 	self.set_flags(self.x - value);
	// 	self.carry_flag = self.x >= value;
	// 	self.pc += 1;
	// }

	fn sbc_absolute_y(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a -= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	// fn cpx_absolute_x(&mut self, ram : MemoryArray) {
	// 	let address: u16 = self.get_address_at_address(ram, self.pc);
	// 	let value: u8 = ram.read(address);
	// 	self.set_flags(self.x - value);
	// 	self.carry_flag = self.x >= value;
	// 	self.pc += 1;
	// }

	fn cmp_absolute_x(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.set_flags(self.a - value);
		self.carry_flag = self.a >= value;
		self.pc += 1;
	}

	fn dec_absolute_x(&mut self, mut ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		ram.write(address, value - 1);
		self.set_flags(value - 1);
		self.pc += 1;
	}

	// fn cpy_zero_page_x(&mut self, ram : MemoryArray) {
	// 	let address: u16 = ram.read(self.pc) as u16;
	// 	let value: u8 = ram.read(address);
	// 	self.set_flags(self.y - value);
	// 	self.carry_flag = self.y >= value;
	// 	self.pc += 1;
	// }

	fn cmp_zero_page_x(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		self.set_flags(self.a - value);
		self.carry_flag = self.a >= value;
		self.pc += 1;
	}

	fn dec_zero_page_x(&mut self, mut ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		ram.write(address, value - 1);
		self.set_flags(value - 1);
		self.pc += 1;
	}

	

	fn cmp_immediate(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.set_flags(self.a - value);
		self.carry_flag = self.a >= value;
		self.pc += 1;
	}

	

	

	// fn bne_relative(&mut self, ram : MemoryArray) {
	// 	let address: u16 = self.get_address_at_address(ram, self.pc);
	// 	if !self.zero_flag {
	// 		self.pc = address;
	// 	} else {
	// 		self.pc += 1;
	// 	}
	// }

	

	fn sbc_zero_page_x(&mut self, ram : MemoryArray) {
		let address: u16 = ram.read(self.pc) as u16;
		let value: u8 = ram.read(address);
		self.a -= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn sed(&mut self) {
		self.decimal_flag = true;
		self.pc += 1;
	}

	fn sbc_absolute(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a -= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn sbc_absolute_x(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a -= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn sta_indirect_y(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a -= value;
		self.set_flags(self.a);
		self.pc += 1;
	}



	fn sbc_indirect_x(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a -= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	fn sbc_indirect_y(&mut self, ram : MemoryArray) {
		let address: u16 = self.get_address_at_address(ram, self.pc);
		let value: u8 = ram.read(address);
		self.a -= value;
		self.set_flags(self.a);
		self.pc += 1;
	}

	
	fn get_flags(&self) -> u8 {
		let mut flags: u8 = 0;
		if self.carry_flag {
			flags |= 0x01;
		}
		if self.zero_flag {
			flags |= 0x02;
		}
		if self.interrupt_flag {
			flags |= 0x04;
		}
		if self.decimal_flag {
			flags |= 0x08;
		}
		if self.break_flag {
			flags |= 0x10;
		}
		if self.overflow_flag {
			flags |= 0x40;
		}
		if self.negative_flag {
			flags |= 0x80;
		}
		flags
	}

	fn set_flags_from_byte(&mut self, flags: u8) {
		self.carry_flag = flags & 0x01 != 0;
		self.zero_flag = flags & 0x02 != 0;
		self.interrupt_flag = flags & 0x04 != 0;
		self.decimal_flag = flags & 0x08 != 0;
		self.break_flag = flags & 0x10 != 0;
		self.overflow_flag = flags & 0x40 != 0;
		self.negative_flag = flags & 0x80 != 0;
	}

	




}