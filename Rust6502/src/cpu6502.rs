// 
// 6502 CPU
//

use crate::memory::MemoryArray;

#[derive(Debug, Copy, Clone)]
pub struct Cpu6502 {
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
	// State
	 pub decimal_mode : bool,
	 pub cycle : u16,
	 // Memory - I found it helped the design a LOT of memory was considered part of the CPU.
	pub memory : MemoryArray,

	pub trace : [u16; 10],
	pub trace_index : u8,
}

impl Default for Cpu6502 {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu6502 {

	pub fn new() -> Cpu6502 {
		Cpu6502 {
			a: 0,
			x: 0,
			y: 0,
			sp: 0xFB, // The default StackPointer. This works for now.
			pc: 0,
			carry_flag: false,
			zero_flag: false,
			overflow_flag: false,
			decimal_flag: false,
			interrupt_flag: false,
			negative_flag: false,
			break_flag: false,
			unused_flag: false,
			decimal_mode : false,
			cycle : 0,
			memory : MemoryArray::init(),
			trace : [0; 10],
			trace_index : 0,
		}
	}

	// Some helper functions.


	pub fn load_data_into_memory(&mut self, address : u16, data : Vec<u8>) {
		for (i, byte) in data.iter().enumerate() {
			self.memory.write(address + i as u16, *byte);
			}
	}

	

	pub fn print_cpu_status_on_one_line(&mut self) {
		print!("Cycle: {:04}  {:04X}  PC: {:04X}  A: {:02X}  X: {:02X}  Y: {:02X}  SP: {:02X}  ", self.cycle, self.memory.read(self.pc+1), self.pc, self.a, self.x, self.y, self.sp);
	
		if self.negative_flag { print!("N"); } else { print!("n"); }
		if self.overflow_flag { print!("V"); } else { print!("v"); }
		print!("_");
		if self.break_flag { print!("B"); } else { print!("b"); }
		if self.decimal_flag { print!("D"); } else { print!("d"); }
		if self.interrupt_flag { print!("I"); } else { print!("i"); }
		if self.zero_flag { print!("Z"); } else { print!("z"); }
		if self.carry_flag { println!("C"); } else { println!("c"); }
		
		self.cycle = self.cycle.wrapping_add(1);

		/* This code waits for a cycle and does a thing */
		/* But need to find the cycle when it quits BASIC and goes to FF00 */
		/*
		if self.cycle == 52
		{

			self.memory.write(0xd011, 0x80);
			self.memory.write(0xd010, 0x48);
			self.memory.apple_key_ready = true;

		}

		if self.cycle == 100
		{
			std::process::exit(0);
		}

		*/
	}

	// Some CPU actions.

	pub fn reset(&mut self) {
		*self = Cpu6502::new();
		self.pc = 0xE000; // Start at software i.e. WozMon or BASIC
	}

	// Apple-1 hardware

	pub fn set_keypress(&mut self, keypress : u8) {
		self.memory.apple_key_ready = true;
		self.memory.apple_key_value = keypress ; 
	}

/*
	pub fn execute_specific_instruction(&mut self, code : u8, mem1: u8, mem2 : u8)
	{
		self.pc = 0;
		self.memory.write(0, code);
		self.memory.write(1, mem1);
		self.memory.write(2, mem2);
		self.execute();
	}
*/
	

	// The call that causes the CPU to execute one instruction.
	// Yeah, it's a giant switch.

	pub fn execute(&mut self) {
		
		let code: u8 = self.memory.read(self.pc);

		// move trace values down array
	    for i in 0..9 {
			self.trace[i] = self.trace[i+1];
		}
		
	self.trace[9] = self.pc;

		
		//print!("  PC: {:#04x}", self.pc);
		//print!("   Code: {:#01x}", code);

		self.pc = self.pc.wrapping_add(1);

		match code {
			0x00 => {
				self.brk();
			}
			0x01 => {
				self.ora_indirect_x();
			}
			0x05 => {
				self.ora_zeropage();
			}
			0x06 => {
				self.asl_zeropage();
			}
			0x08 => {
				self.php();
			}
			0x09 => {
				self.ora_immediate();
			}
			0x0a => {
				self.asl_accumulator();
			}
			0x0d => {
				self.ora_absolute();
			}
			0x0e => {
				self.asl_absolute();
			}
			0x10 => {
				self.bpl();
			}
			0x11 => {
				self.ora_indirect();
			}
			0x15 => {
				self.ora_zeropage_x();
			}
			0x16 => {
				self.asl_zeropage_x();
			}
			0x18 => {
				self.clc();
			}
			0x19 => {
				self.ora_absolute_y();
			}

			0x1d => {
				self.ora_absolute_x();
			}
			0x1e => {
				self.asl_absolute_x();
			}
			0x20 => {
				self.jsr();
			}
			0x21 => {
				self.and_indirect_x();
			}
			0x24 => {
				self.bit_zeropage();
			}

			0x25 => {
				self.and_zeropage();
			}

			0x26 => {
				self.rol_zeropage();
			}

			0x28 => {
				self.plp();
			}

			0x29 => {
				self.and_immediate();
			}

			0x2a => {
				self.rol_accumulator();
			}

			0x2c => {
				self.bit_absolute();
			}

			0x2d => {
				self.and_absolute();
			}

			0x2e => {
				self.rol_absolute();
			}

			0x30 => {
				self.bmi();
			}

			0x31 => {
				self.and_indirect_y();
			}

			0x35 => {
				self.and_zeropage_x();
			}

			0x36 => {
				self.rol_zeropage_x();
			}

			0x38 => {
				self.sec();
			}

			0x39 => {
				self.and_absolute_y();
			}

			0x3d => {
				self.and_absolute_x();
			}

			0x3e => {
				self.rol_absolute_x();
			}

			0x40 => {
				self.rti();
			}
			
			
			0x41 => {
				self.eor_indirect_x();
			}

			0x45 => {
				self.eor_zeropage();
			}

			0x46 => {
				self.lsr_zeropage();
			}

			0x48 => {
				self.pha();
			}

			0x49 => {
				self.eor_immediate();
			}

			0x4a => {
				self.lsr_accumulator();
			}

			0x4c => {
				self.jmp_absolute();
			}

			0x4d => {
				self.eor_absolute();
			}

			0x4e => {
				self.lsr_absolute();
			}

			0x50 => {
				self.bvc();
			}

			0x51 => {
				self.eor_indirect_y();
			}

			0x55 => {
				self.eor_zeropage_x();
			}

			0x56 => {
				self.lsr_zeropage_x();
			}

			0x58 => {
				self.cli();
			}

			0x59 => {
				self.eor_absolute_y();
			}

			0x5d => {
				self.eor_absolute_x();
			}

			0x5e => {
				self.lsr_absolute_x();
			}

			0x60 => {
				self.rts();
			}

			0x61 => {
				self.adc_indirect_x();
			}

			0x65 => {
				self.adc_zeropage();
			}

			0x66 => {
				self.ror_zeropage();
			}

			0x68 => {
				self.pla();
			}

			0x69 => {
				self.adc_immediate();
			}

			0x6a => {
				self.ror_accumulator();
			}

			0x6c => {
				self.jmp_indirect();
			}

			0x6d => {
				self.adc_absolute();
			}

			0x6e => {
				self.ror_absolute();
			}

			0x70 => {
				self.bvs();
			}

			0x71 => {
				self.adc_indirect_y();
			}

			0x75 => {
				self.adc_zeropage_x();
			}

			0x76 => {
				self.ror_zeropage_x();
			}

			0x78 => {
				self.sei();
			}

			0x79 => {
				self.adc_absolute_y();
			}

			0x7d => {
				self.adc_absolute_x();
			}

			0x7e => {
				self.ror_absolute_x();
			}

			0x80 => {
				self.bra();
			}


			0x81 => {
				self.sta_indirect_x();
			}

			0x84 => {
				self.sty_zeropage();
			}

			0x85 => {
				self.sta_zeropage();
			}

			0x86 => {
				self.stx_zeropage();
			}

			0x88 => {
				self.dey();
			}

			0x8a => {
				self.txa();
			}

			0x8c => {
				self.sty_absolute();
			}

			0x8d => {
				self.sta_absolute();
			}

			0x8e => {
				self.stx_absolute();
			}

			0x90 => {
				self.bcc();
			}

			0x91 => {
				self.sta_indirect_y();
			}

			0x94 => {
				self.sty_zeropage_x();
			}

			0x95 => {
				self.sta_zeropage_x();
			}

			0x96 => {
				self.stx_zeropage_y();
			}

			0x98 => {
				self.tya();
			}

			0x99 => {
				self.sta_absolute_y();
			}

			0x9a => {
				self.txs();
			}

			0x9d => {
				self.sta_absolute_x();
			}

			0xa0 => {
				//println!("LDY");
				self.ldy_immediate();
			}

			0xa1 => {
				self.lda_indirect_x();
			}

			0xa2 => {
				self.ldx_immediate();
			}

			0xa4 => {
				self.ldy_zeropage();
			}

			0xa5 => {
				self.lda_zeropage();
			}

			0xa6 => {
				self.ldx_zeropage();
			}

			0xa8 => {
				self.tay();
			}

			0xa9 => {
				self.lda_immediate();
			}

			0xaa => {
				self.tax();
			}

			0xac => {
				self.ldy_absolute();
			}

			0xad => {
				self.lda_absolute();
			}

			0xae => {
				self.ldx_absolute();
			}

			0xb0 => {
				self.bcs();
			}

			0xb1 => {
				self.lda_indirect_y();
			}

			0xb4 => {
				self.ldy_zeropage_x();
			}

			0xb5 => {
				self.lda_zeropage_x();
			}

			0xb6 => {
				self.ldx_zeropage_y();
			}

			0xb8 => {
				self.clv();
			}

			0xb9 => {
				self.lda_absolute_y();
			}

			0xba => {
				self.tsx();
			}

			0xbc => {
				self.ldy_absolute_x();
			}

			0xbd => {
				self.lda_absolute_x();
			}

			0xbe => {
				self.ldx_absolute_y();
			}

			0xc0 => {
				self.cpy_immediate();
			}

			0xc1 => {
				self.cmp_indirect_x();
			}

			0xc4 => {
				self.cpy_zeropage();
			}

			0xc5 => {
				self.cmp_zeropage();
			}

			0xc6 => {
				self.dec_zeropage();
			}

			0xc8 => {
				self.iny();
			}

			0xc9 => {
				self.cmp_immediate();
			}

			0xca => {
				self.dex();
			}

			0xcc => {
				self.cpy_absolute();
			}

			0xcd => {
				self.cmp_absolute();
			}

			0xce => {
				self.dec_absolute();
			}

			0xd0 => {
				self.bne();
			}

			0xd1 => {
				self.cmp_indirect_y();
			}

			0xd5 => {
				self.cmp_zeropage_x();
			}

			0xd6 => {
				self.dec_zeropage_x();
			}

			0xd8 => {
				self.cld();
			}

			0xd9 => {
				self.cmp_absolute_y();
			}

			0xdd => {
				self.cmp_absolute_x();
			}

			0xde => {
				self.dec_absolute_x();
			}

			0xe0 => {
				self.cpx_immediate();
			}

			0xe1 => {
				self.sbc_indirect_x();
			}

			0xe4 => {
				self.cpx_zeropage();
			}

			0xe5 => {
				self.sbc_zeropage();
			}

			0xe6 => {
				self.inc_zeropage();
			}

			0xe8 => {
				self.inx();
			}

			0xe9 => {
				self.sbc_immediate();
			}

			0xea => {
				self.nop();
			}

			0xec => {
				self.cpx_absolute();
			}

			0xed => {
				self.sbc_absolute();
			}

			0xee => {
				self.inc_absolute();
			}

			0xf0 => {
				self.beq();
			}

			0xf1 => {
				self.sbc_indirect_y();
			}

			0xf5 => {
				self.sbc_zeropage_x();
			}

			0xf6 => {
				self.inc_zeropage_x();
			}

			0xf8 => {
				self.sed();
			}

			0xf9 => {
				self.sbc_absolute_y();
			}

			0xfd => {
				self.sbc_absolute_x();
			}

			0xfe => {
				self.inc_absolute_x();
			}

			_ => {


				for p in 0 .. 10 {
					println!("Backtrace {:04X} {:02X}", self.trace[p], self.memory.read(self.trace[p]));
				}
				panic!("Invalid opcode: {:x} at address {:04X}", code,self.pc );

			}
			
		}
	
		// print!("{:02x}  {:04X}  PC: {:04X}  A: {:02X}  X: {:02X}  Y: {:02X}  SP: {:02X}  ", code, self.memory.read(self.pc), self.pc, self.a, self.x, self.y, self.sp);
	
		// if self.negative_flag { print!("N"); } else { print!("n"); }
		// if self.overflow_flag { print!("V"); } else { print!("v"); }
		// print!("_");
		// if self.break_flag { print!("B"); } else { print!("b"); }
		// if self.decimal_flag { print!("D"); } else { print!("d"); }
		// if self.interrupt_flag { print!("I"); } else { print!("i"); }
		// if self.zero_flag { print!("Z"); } else { print!("z"); }
		// if self.carry_flag { println!("C"); } else { println!("c"); }
		
	}

	

	// More helpers for getting addresses, doing math etc.
	// NB not proven to work just yet.

	fn compare(&mut self, a: u8, b: u8) {

		let result = (a as u16).wrapping_sub(b as u16);

		if a >= (b & 0xFF) { self.carry_flag = true } else { self.carry_flag = false }
        if a == (b & 0xFF) { self.zero_flag = true } else { self.zero_flag = false }
		if (result & 0x80) == 0x80 {self.negative_flag = true } else { self.negative_flag = false }


		// let result = a.wrapping_sub(b);
		// if result == 0 {self.zero_flag = true;} else {self.zero_flag = false;}
		// if a >= b {self.carry_flag = true;} else {self.carry_flag = false;}
		// if (a & 0x80) != (b & 0x80) && (a & 0x80) != (result & 0x80) {self.overflow_flag = true;} else {self.overflow_flag = false;}
		// if result & 0x80 == 0x80 {self.negative_flag = true;} else {self.negative_flag = false;}
	}
	
	fn set_flags(&mut self, value : u8) {
		self.zero_flag = value == 0;
		self.negative_flag = value & 0x80 != 0;
	}

	fn get_address_at_address(&mut self, address : u16) -> u16 {
		let low_byte = self.memory.read(address) as u16;
		let high_byte = self.memory.read(address + 1) as u16;
		(high_byte << 8) | low_byte
	}

	fn get_absolute_address(&mut self) -> u16 {
		let low_byte = self.memory.read(self.pc) as u16;
		let high_byte = self.memory.read(self.pc + 1) as u16;
		(high_byte << 8) | low_byte
		
	}

	fn get_relative(&mut self) -> u8 {
		let value = self.memory.read(self.pc) as u8;
		value
	}

	fn subtract_with_carry_decimal(&mut self, value: u8) {
		let  total : u16 ;
		let mut bcd_low : u16 ;
		let mut bcd_high : u16 ;

		let operand0 : u8 ;
		let operand1 : u8 ;
		let result : u8 ;
		let flag_c_invert : u8 ;
		let mut low_carry : u8 = 0;
		let mut high_carry : u8 = 0;
		let register_a = self.a;

		if self.carry_flag  {
			flag_c_invert = 0;
		} else {
			flag_c_invert = 1;
		}

		if self.decimal_mode {

			bcd_low = 0xffff & (0x0f & register_a) as u16 - (0x0f & value) as u16 - flag_c_invert as u16;
			if bcd_low > 0x09 {
				low_carry = 0x10;
				bcd_low = bcd_low.wrapping_add(0x0A);
			}

			bcd_high = 0xffff & (0xf0 & register_a) as u16 - (0xf0 & value) as u16 - low_carry as u16;

			if bcd_high > 0x90 {
				high_carry = 1;
				bcd_high = bcd_high.wrapping_add(0xA0);
			}

			self.carry_flag = false;

			if high_carry == 0 {
				self.carry_flag = true;
			}

			total = bcd_low | bcd_high;


		} else {

			total = register_a.wrapping_sub(value as u8).wrapping_sub(flag_c_invert as u8) as u16;
			
			if total > 0xff {
				self.carry_flag = false;
			} else {
				self.carry_flag = true;
			}
		}

		operand0 = register_a & 0x80;
		operand1 = value & 0x80;
		result = (total & 0x80) as u8;

		if operand0 == 0 && operand1 != 0 && result != 0 {
			self.overflow_flag = true;
		} else if operand0 != 0 && operand1 == 0 && result == 0 {
			self.overflow_flag = true;
		} else {
			self.overflow_flag = false;
		}

		self.a = (total & 0xff) as u8;

		self.set_flags(self.a);
	}

	

	fn get_absolute_address_x(&mut self) -> u16 {
		 self.get_absolute_address().wrapping_add(self.x as u16)
		 		
	}

	fn get_absolute_address_y(&mut self) -> u16 {
		 self.get_absolute_address().wrapping_add(self.y as u16)
		
	}

	fn get_zeropage(&mut self) -> u16 {
		self.memory.read(self.pc) as u16
	}

	fn get_zeropage_x(&mut self) -> u16 {
		self.memory.read(self.pc).wrapping_add(self.x) as u16
	}

	fn get_zeropage_y(&mut self) -> u16 {
	 	self.memory.read(self.pc).wrapping_add(self.y) as u16
	}

	fn get_immediate(&mut self) -> u8 {
		self.memory.read(self.pc )
	}

	fn get_indirect_x(&mut self) -> u16 {
		let address = self.memory.read(self.pc).wrapping_add(self.x) as u16;
		let low_byte = self.memory.read(address) as u16;
		let high_byte = self.memory.read(address.wrapping_add(1)) as u16;
		(high_byte << 8) | low_byte
		
	}

	fn get_indirect_y(&mut self) -> u16 {
		let address = self.memory.read(self.pc).wrapping_add(self.y) as u16;
		let low_byte = self.memory.read(address) as u16;
		let high_byte = self.memory.read(address.wrapping_add(1)) as u16;
		(high_byte << 8) | low_byte		
	}

	fn get_indirect(&mut self) -> u16 {
		let address = self.get_absolute_address();
		let low_byte = self.memory.read(address) as u16;
		let high_byte = self.memory.read(address.wrapping_add(1)) as u16;
		(high_byte << 8) | low_byte
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

	fn push_stack(&mut self, value : u8) {
		self.memory.write( 0x0100 + self.sp as u16, value);
		self.sp = self.sp.wrapping_sub(1);
	}

	fn pop_stack(&mut self) -> u8 {
		self.sp = self.sp.wrapping_add(1);
		self.memory.read(0x0100 + self.sp as u16)
	}



	// 6502 Instruction Set
	// Again not yet proven, there will be bugs. Needs testing.
	// Likely issues are around the addressing, math and PC incrementing.



	fn brk(&mut self) {
		//print!("brk");
		self.pc = self.pc.wrapping_sub(1);
		let h: u8 = (self.pc >> 8) as u8; self.push_stack(h);
		let l: u8 = (self.pc & 0xff) as u8; self.push_stack(l);
		let sr : u8 = self.get_status_register(); self.push_stack(sr);
		//self.pc = self.get_address_at_address(0x17fa);
		//self.pc = 0xfffe;
	}

	fn ora_indirect_x(&mut self) {
		let address = self.get_indirect_x();
		let value = self.memory.read(address);
		self.a |= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn ora_zeropage(&mut self) {
		let address = self.get_zeropage();
		let value = self.memory.read(address);
		self.a |= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn asl_zeropage(&mut self) {
		let address = self.get_zeropage();
		let mut value = self.memory.read(address);
		value <<= 1;
		self.carry_flag = (value & 128) == 128;
		self.memory.write(address, value);
		self.set_flags(value);
		self.pc = self.pc.wrapping_add(1);
	}


	fn php(&mut self) {
		let sr: u8 = self.get_status_register();
		self.push_stack(sr);
	}


	fn ora_immediate(&mut self) {
		let value: u8 = self.memory.read(self.pc);
		self.a |= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn asl_accumulator(&mut self) {
		let result: u8 = self.a << 1;
		self.carry_flag = (self.a & 128) == 128;
		self.a = result;
	}

	fn ora_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		let value: u8 = self.memory.read(address);
		self.a |= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(2);
	}

	fn asl_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		let value: u8 = self.memory.read(address);
		let result: u8 = value << 1;
		self.carry_flag = (value & 128) == 128;
		self.set_flags(result);
		self.memory.write(address, result);
		self.pc = self.pc.wrapping_add(2);
	}

	fn bra(&mut self) {
		let offset: u8 = self.memory.read(self.pc);
		self.perform_relative_address(offset);
	}

	fn bmi(&mut self) {
		let offset: u8 = self.memory.read(self.pc);
		if self.negative_flag {
			self.perform_relative_address(offset);
		} else {
			self.pc = self.pc.wrapping_add(1);
		}
	}

	fn bpl(&mut self) {
		let offset: u8 = self.memory.read(self.pc);
		if !self.negative_flag {
			self.perform_relative_address(offset)
		}
		else {
			self.pc = self.pc.wrapping_add(1);
		}
	}

	fn ora_indirect(&mut self) {
		let address: u16 = self.get_indirect();
		let value: u8 = self.memory.read(address);
		self.a |= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn ora_zeropage_x(&mut self) {
		let address: u16 = self.get_zeropage_x();
		let value: u8 = self.memory.read(address);
		self.a |= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn asl_zeropage_x(&mut self) {
		let address: u16 = self.get_zeropage_x();
		let value: u8 = self.memory.read(address);
		let result: u8 = value << 1;
		self.set_flags(result);
		self.carry_flag = (value & 128) == 128;
		self.memory.write(address, result);
		self.pc = self.pc.wrapping_add(1);
	}

	fn clc(&mut self) {
		self.carry_flag = false;
	}

	fn ora_absolute_y(&mut self) {
		let address: u16 = self.get_absolute_address_y();
		let value: u8 = self.memory.read(address);
		self.a |= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(2);
	}


	fn ora_absolute_x(&mut self) {
		let address: u16 = self.get_absolute_address_x();
		let value: u8 = self.memory.read(address);
		self.a |= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(2);
	}

	fn asl_absolute_x(&mut self) {
		let address: u16 = self.get_absolute_address_x();
		let value: u8 = self.memory.read(address);
		let result: u8 = value << 1;
		self.carry_flag = (value & 128) == 128;
		self.set_flags(result);
		self.memory.write(address, result);
		self.pc = self.pc.wrapping_add(2);
	}

	fn jsr(&mut self) { 
		let address: u16 = self.get_absolute_address(); 
		self.pc = self.pc.wrapping_add(2); // adding 2?
		let h: u8 = (self.pc >> 8) as u8; self.push_stack(h);
		let l: u8 = (self.pc & 0xff) as u8; self.push_stack(l);
		self.pc = address;
	}

	fn and_indirect_x(&mut self) {
		let address: u16 = self.get_indirect_x();
		let value: u8 = self.memory.read(address);
		self.a &= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn bit_zeropage(&mut self) {
		let address: u16 = self.get_zeropage();
		let value: u8 = self.memory.read(address);
		self.zero_flag = self.a & value == 0;
		self.negative_flag = value & 0x80 != 0;
		self.overflow_flag = value & 0x40 != 0;
		self.pc = self.pc.wrapping_add(1);
	}

	fn and_zeropage(&mut self) {
		let address: u16 = self.get_zeropage();
		let value: u8 = self.memory.read(address);
		self.a &= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn rol_zeropage(&mut self) {
		let address: u16 = self.get_zeropage();
		let mut value: u8 = self.memory.read(address);
		let carry: u8 = if self.carry_flag { 1 } else { 0 };
		self.carry_flag = value & 0x80 != 0;
		value <<= 1;
		value |= carry;
		self.memory.write(address, value);
		self.set_flags(value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn plp(&mut self) {
		let value: u8 = self.pop_stack();
		self.negative_flag = value & 0x80 != 0;
		self.overflow_flag = value & 0x40 != 0;
		self.break_flag = value & 0x10 != 0;
		self.decimal_flag = value & 0x08 != 0;
		self.interrupt_flag = value & 0x04 != 0;
		self.zero_flag = value & 0x02 != 0;
		self.carry_flag = value & 0x01 != 0;
	}

	fn and_immediate(&mut self) {
		let value: u8 = self.memory.read(self.pc);
		self.a &= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn rol_accumulator(&mut self) {
		let carry: u8 = if self.carry_flag { 1 } else { 0 };
		self.carry_flag = self.a & 0x80 != 0;
		self.a <<= 1;
		self.a |= carry;
		self.set_flags(self.a);
	}

	fn sta_indirect_y(&mut self) {
		let address: u16 = self.get_indirect_y();
		self.memory.write(address, self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn bit_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		let value: u8 = self.memory.read(address);
		self.zero_flag = self.a & value == 0;
		self.negative_flag = value & 0x80 != 0;
		self.overflow_flag = value & 0x40 != 0;
		self.pc = self.pc.wrapping_add(2);
	}

	fn and_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		let value: u8 = self.memory.read(address);
		self.a &= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(2);
	}

	fn rol_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		let mut value: u8 = self.memory.read(address);
		let carry: u8 = if self.carry_flag { 1 } else { 0 };
		self.carry_flag = value & 0x80 != 0;
		value <<= 1;
		value |= carry;
		self.memory.write(address, value);
		self.set_flags(value);
		self.pc = self.pc.wrapping_add(2);
	}

	fn and_indirect_y(&mut self) {
		let address: u16 = self.get_indirect_y();
		let value: u8 = self.memory.read(address);
		self.a &= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn and_zeropage_x(&mut self) {
		let address: u16 = self.get_zeropage_x();
		let value: u8 = self.memory.read(address);
		self.a &= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn rol_zeropage_x(&mut self) {
		let address: u16 = self.get_zeropage_x();
		let mut value: u8 = self.memory.read(address);
		let carry: u8 = if self.carry_flag { 1 } else { 0 };
		self.carry_flag = value & 0x80 != 0;
		value <<= 1;
		value |= carry;
		self.memory.write(address, value);
		self.set_flags(value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn sec(&mut self) {
		self.carry_flag = true;
	}

	fn and_absolute_y(&mut self) {
		let address: u16 = self.get_absolute_address_y();
		let value: u8 = self.memory.read(address);
		self.a &= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(2);
	}

	fn and_absolute_x(&mut self) {
		let address: u16 = self.get_absolute_address_x();
		let value: u8 = self.memory.read(address);
		self.a &= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(2);
	}

	fn rol_absolute_x(&mut self) {
		let address: u16 = self.get_absolute_address_x();
		let mut value: u8 = self.memory.read(address);
		let carry: u8 = if self.carry_flag { 1 } else { 0 };
		self.carry_flag = value & 0x80 != 0;
		value <<= 1;
		value |= carry;
		self.memory.write(address, value);
		self.set_flags(value);
		self.pc = self.pc.wrapping_add(2);
	}

	fn rti(&mut self) {
		let value: u8 = self.pop_stack();
		self.negative_flag = value & 0x80 != 0;
		self.overflow_flag = value & 0x40 != 0;
		self.break_flag = value & 0x10 != 0;
		self.decimal_flag = value & 0x08 != 0;
		self.interrupt_flag = value & 0x04 != 0;
		self.zero_flag = value & 0x02 != 0;
		self.carry_flag = value & 0x01 != 0;
		let low: u8 = self.pop_stack();
		let high: u8 = self.pop_stack();
		self.pc = ((high as u16) << 8) | low as u16;
	}

	fn eor_indirect_x(&mut self) {
		let address: u16 = self.get_indirect_x();
		let value: u8 = self.memory.read(address);
		self.a ^= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn eor_zeropage(&mut self) {
		let address: u16 = self.get_zeropage();
		let value: u8 = self.memory.read(address);
		self.a ^= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn lsr_zeropage(&mut self) {
		let address: u16 = self.get_zeropage();
		let mut value: u8 = self.memory.read(address);
		self.carry_flag = value & 0x01 != 0;
		value >>= 1;
		self.memory.write(address, value);
		self.set_flags(value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn pha(&mut self) {
		self.push_stack(self.a);
	}

	fn eor_immediate(&mut self) {
		let value: u8 = self.get_immediate();
		self.a ^= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn lsr_accumulator(&mut self) {
		self.carry_flag = self.a & 0x01 != 0;
		self.a >>= 1;
		self.set_flags(self.a);
	}

	fn jmp_absolute(&mut self) {
		let address: u16 = self.get_address_at_address(self.pc);
		self.pc = address;
	}

	fn eor_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		let value: u8 = self.memory.read(address);
		self.a ^= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(2);
	}

	fn lsr_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		let mut value: u8 = self.memory.read(address);
		self.carry_flag = value & 0x01 != 0;
		value >>= 1;
		self.memory.write(address, value);
		self.set_flags(value);
		self.pc = self.pc.wrapping_add(2);
	}

	fn perform_relative_address(&mut self, offset: u8) {

		let mut t = offset as u16;
		let mut address = self.pc.wrapping_add(t as u16);
		if t & 0x80 == 0x80 {
			t = 0x100 - t;
			address = self.pc.wrapping_sub(t as u16);
		}

		self.pc = address.wrapping_add(1);
	}

	fn bvc(&mut self) {
		let offset: u8 = self.get_relative();
		if !self.overflow_flag {
			self.perform_relative_address(offset);
		} else {
			self.pc = self.pc.wrapping_add(1);
		}
	}

	fn eor_indirect_y(&mut self) {
		let address: u16 = self.get_indirect_y();
		let value: u8 = self.memory.read(address);
		self.a ^= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn eor_zeropage_x(&mut self) {
		let address: u16 = self.get_zeropage_x();
		let value: u8 = self.memory.read(address);
		self.a ^= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(1);
	}
	
	fn lsr_zeropage_x(&mut self) {
		let address: u16 = self.get_zeropage_x();
		let mut value: u8 = self.memory.read(address);
		self.carry_flag = value & 0x01 != 0;
		value >>= 1;
		self.memory.write(address, value);
		self.set_flags(value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn cli(&mut self) {
		self.interrupt_flag = false;
	}


	fn eor_absolute_y(&mut self) {
		let address: u16 = self.get_absolute_address_y();
		let value: u8 = self.memory.read(address);
		self.a ^= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(2);
	}



	fn eor_absolute_x(&mut self) {
		let address: u16 = self.get_absolute_address_x();
		let value: u8 = self.memory.read(address);
		self.a ^= value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(2);
	}

	fn lsr_absolute_x(&mut self) {
		let address: u16 = self.get_absolute_address_x();
		let mut value: u8 = self.memory.read(address);
		self.carry_flag = value & 0x01 != 0;
		value >>= 1;
		self.memory.write(address, value);
		self.set_flags(value);
		self.pc = self.pc.wrapping_add(2);
	}

	fn rts(&mut self) {
		let address_l: u16 = self.pop_stack() as u16;
		let address_h: u16 = self.pop_stack() as u16;
		let address = (address_h << 8) | address_l;
		self.pc = address;
	}

	fn adc_indirect_x(&mut self) {
		let address: u16 = self.get_indirect_x();
		let value: u8 = self.memory.read(address);
		self.adc(value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn adc_zeropage(&mut self) {
		let address: u16 = self.get_zeropage();
		let value: u8 = self.memory.read(address);
		self.adc(value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn ror_zeropage(&mut self) {
		let address: u16 = self.get_zeropage();
		let mut value: u8 = self.memory.read(address);
		let carry: u8 = if self.carry_flag { 0x80 } else { 0x00 };
		self.carry_flag = value & 0x01 != 0;
		value >>= 1;
		value |= carry;
		self.memory.write(address, value);
		self.set_flags(value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn pla(&mut self) {
		self.a = self.pop_stack();
		self.set_flags(self.a);
	}

	fn adc_immediate(&mut self) {
		let value: u8 = self.get_immediate();
		self.adc(value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn ror_accumulator(&mut self) {

		let lsb = (self.a & 0x01)  == 1;
		self.a >>= 1;
		self.a = self.a | ((self.carry_flag as u8) << 7);
		self.set_flags(self.a);
		self.carry_flag = lsb;
	}

	fn jmp_indirect(&mut self) {
		let address: u16 = self.get_indirect();
		self.pc = address;
	}


	fn adc_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		let value: u8 = self.memory.read(address);
		self.adc(value);
		self.pc = self.pc.wrapping_add(2);
	}

	fn ror_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		let mut value: u8 = self.memory.read(address);
		let carry: u8 = if self.carry_flag { 0x80 } else { 0x00 };
		self.carry_flag = value & 0x01 != 0;
		value >>= 1;
		value |= carry;
		self.memory.write(address, value);
		self.set_flags(value);
		self.pc = self.pc.wrapping_add(2);
	}

	fn bvs(&mut self) {
		let offset: u8 = self.get_relative();
		if self.overflow_flag {
			self.perform_relative_address(offset);
		} else {
			self.pc = self.pc.wrapping_add(1);
		}
	}

	fn adc_indirect_y(&mut self) {
		let address: u16 = self.get_indirect_y();
		let value: u8 = self.memory.read(address);
		self.adc(value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn adc_zeropage_x(&mut self) {
		let address: u16 = self.get_zeropage_x();
		let value: u8 = self.memory.read(address);
		self.adc(value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn ror_zeropage_x(&mut self) {
		let address: u16 = self.get_zeropage_x();
		let mut value: u8 = self.memory.read(address);
		let carry: u8 = if self.carry_flag { 0x80 } else { 0x00 };
		self.carry_flag = value & 0x01 != 0;
		value >>= 1;
		value |= carry;
		self.memory.write(address, value);
		self.set_flags(value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn sei(&mut self) {
		self.interrupt_flag = true;
	}

	fn adc_absolute_y(&mut self) {
		let address: u16 = self.get_absolute_address_y();
		let value: u8 = self.memory.read(address);
		self.adc(value);
		self.pc = self.pc.wrapping_add(2);
	}

	fn adc_absolute_x(&mut self) {
		let address: u16 = self.get_absolute_address_x();
		let value: u8 = self.memory.read(address);
		self.adc(value);
		self.pc = self.pc.wrapping_add(2);
	}

	fn ror_absolute_x(&mut self) {
		let address: u16 = self.get_absolute_address_x();
		let mut value: u8 = self.memory.read(address);
		let carry: u8 = if self.carry_flag { 0x80 } else { 0x00 };
		self.carry_flag = value & 0x01 != 0;
		value >>= 1;
		value |= carry;
		self.memory.write(address, value);
		self.set_flags(value);
		self.pc = self.pc.wrapping_add(2);
	}

	fn sta_indirect_x(&mut self) {
		let address: u16 = self.get_indirect_x();
		self.memory.write(address, self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn sty_zeropage(&mut self) {
		let address: u16 = self.get_zeropage();
		self.memory.write(address, self.y);
		self.pc = self.pc.wrapping_add(1);
	}

	fn sta_zeropage(&mut self) {
		let address: u16 = self.get_zeropage();
		self.memory.write(address, self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn stx_zeropage(&mut self) {
		let address: u16 = self.get_zeropage();
		self.memory.write(address, self.x);
		self.pc = self.pc.wrapping_add(1);
	}

	fn dey(&mut self) {
		self.y = self.y.wrapping_sub(1);
		self.set_flags(self.y);
	}

	fn txa(&mut self) {
		self.a = self.x;
		self.set_flags(self.a);
	}

	fn sty_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		self.memory.write(address, self.y);
		self.pc = self.pc.wrapping_add(2);
	}

	fn sta_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		self.memory.write(address, self.a);
		self.pc = self.pc.wrapping_add(2);
	}

	fn stx_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		self.memory.write(address, self.x);
		self.pc = self.pc.wrapping_add(2);
	}

	fn bcc(&mut self) {
		let offset: u8 = self.get_relative();
		if !self.carry_flag {
			self.perform_relative_address(offset);
		} else {
			self.pc = self.pc.wrapping_add(1);
		}
	}

	fn sty_zeropage_x(&mut self) {
		let address: u16 = self.get_zeropage_x();
		self.memory.write(address, self.y);
		self.pc = self.pc.wrapping_add(1);
	}

	fn sta_zeropage_x(&mut self) {
		let address: u16 = self.get_zeropage_x();
		self.memory.write(address, self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn stx_zeropage_y(&mut self) {
		let address: u16 = self.get_zeropage_y();
		self.memory.write(address, self.x);
		self.pc = self.pc.wrapping_add(1);
	}

	fn tya(&mut self) {
		self.a = self.y;
		self.set_flags(self.a);
	}

	fn sta_absolute_y(&mut self) {
		let address: u16 = self.get_absolute_address_y();
		self.memory.write(address, self.a);
		self.pc = self.pc.wrapping_add(2);
	}

	fn sta_absolute_x(&mut self) {
		let address: u16 = self.get_absolute_address_x();
		self.memory.write(address, self.a);
		self.pc = self.pc.wrapping_add(2);
	}

	fn ldy_immediate(&mut self) {
		let value: u8 = self.get_immediate();
		self.y = value;
		self.set_flags(self.y);
		self.pc = self.pc.wrapping_add(1);
	}

	fn lda_indirect_x(&mut self) {
		let address: u16 = self.get_indirect_x();
		let value: u8 = self.memory.read(address);
		self.a = value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn ldx_immediate(&mut self) {
		let value: u8 = self.get_immediate();
		self.x = value;
		self.set_flags(self.x);
		self.pc = self.pc.wrapping_add(1);
	}

	fn ldy_zeropage(&mut self) {
		let address: u16 = self.get_zeropage();
		let value: u8 = self.memory.read(address);
		self.y = value;
		self.set_flags(self.y);
		self.pc = self.pc.wrapping_add(1);
	}

	fn lda_zeropage(&mut self) {
		let address: u16 = self.get_zeropage();
		let value: u8 = self.memory.read(address);
		self.a = value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn ldx_zeropage(&mut self) {
		let address: u16 = self.get_zeropage();
		let value: u8 = self.memory.read(address);
		self.x = value;
		self.set_flags(self.x);
		self.pc = self.pc.wrapping_add(1);
	}

	fn tay(&mut self) {
		self.y = self.a;
		self.set_flags(self.y);
	}

	fn lda_immediate(&mut self) {
		let value: u8 = self.get_immediate();
		self.a = value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn txs(&mut self) {
		self.sp = self.x;
	}

	fn tax(&mut self) {
		self.x = self.a;
		self.set_flags(self.x);
	}

	fn beq(&mut self) {
		let offset: u8 = self.get_relative();
		if self.zero_flag {
			self.perform_relative_address(offset);
		} else {
			self.pc = self.pc.wrapping_add(1);
		}
	}


	fn ldy_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		let value: u8 = self.memory.read(address);
		self.y = value;
		self.set_flags(self.y);
		self.pc = self.pc.wrapping_add(2);
	}

	fn lda_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		let value: u8 = self.memory.read(address);
		self.a = value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(2);
	}

	fn ldx_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		let value: u8 = self.memory.read(address);
		self.x = value;
		self.set_flags(self.x);
		self.pc = self.pc.wrapping_add(2);
	}

	fn bcs(&mut self) {
		let offset: u8 = self.get_relative();
		if self.carry_flag {
			self.perform_relative_address(offset);
		} else {
			self.pc = self.pc.wrapping_add(1);
		}
	}

	fn lda_indirect_y(&mut self) {
		let address: u16 = self.get_indirect_y();
		let value: u8 = self.memory.read(address);
		self.a = value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn ldy_zeropage_x(&mut self) {
		let address: u16 = self.get_zeropage_x();
		let value: u8 = self.memory.read(address);
		self.y = value;
		self.set_flags(self.y);
		self.pc = self.pc.wrapping_add(1);
	}

	fn lda_zeropage_x(&mut self) {
		let address: u16 = self.get_zeropage_x();
		let value: u8 = self.memory.read(address);
		self.a = value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(1);
	}

	fn ldx_zeropage_y(&mut self) {
		let address: u16 = self.get_zeropage_y();
		let value: u8 = self.memory.read(address);
		self.x = value;
		self.set_flags(self.x);
		self.pc = self.pc.wrapping_add(1);
	}

	fn clv(&mut self) {
		self.overflow_flag = false;
	}

	fn tsx(&mut self) {
		self.x = self.sp;
		self.set_flags(self.x);
	}

	fn ldy_absolute_x(&mut self) {
		let address: u16 = self.get_absolute_address_x();
		let value: u8 = self.memory.read(address);
		self.y = value;
		self.set_flags(self.y);
		self.pc = self.pc.wrapping_add(2);
	}

	fn lda_absolute_y(&mut self) {
		let address: u16 = self.get_absolute_address_y();
		let value: u8 = self.memory.read(address);
		self.a = value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(2);
	}

	fn lda_absolute_x(&mut self) {
		let address: u16 = self.get_absolute_address_x();
		let value: u8 = self.memory.read(address);
		self.a = value;
		self.set_flags(self.a);
		self.pc = self.pc.wrapping_add(2);
	}

	fn ldx_absolute_y(&mut self) {
		let address: u16 = self.get_absolute_address_y();
		let value: u8 = self.memory.read(address);
		self.x = value;
		self.set_flags(self.x);
		self.pc = self.pc.wrapping_add(2);
	}

	fn cpy_immediate(&mut self) {
		let value: u8 = self.get_immediate();
		self.compare(self.y, value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn cmp_indirect_x(&mut self) {
		let address: u16 = self.get_indirect_x();
		let value: u8 = self.memory.read(address);
		self.compare(self.a, value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn cpy_zeropage(&mut self) {
		let address: u16 = self.get_zeropage();
		let value: u8 = self.memory.read(address);
		self.compare(self.y, value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn cmp_zeropage(&mut self) {
		let address: u16 = self.get_zeropage();
		let value: u8 = self.memory.read(address);
		self.compare(self.a, value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn dec_zeropage(&mut self) {
		let address: u16 = self.get_zeropage();
		let value: u8 = self.memory.read(address);
		let result: u8 = value.wrapping_sub(1);
		self.memory.write(address, result);
		self.set_flags(result);
		self.pc = self.pc.wrapping_add(1);
	}

	fn iny(&mut self) {
		self.y = self.y.wrapping_add(1);
		self.set_flags(self.y);
	}

	fn cmp_immediate(&mut self) {
		let value: u8 = self.get_immediate();
		self.compare(self.a, value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn dex(&mut self) {
		self.x = self.x.wrapping_sub(1);
		self.set_flags(self.x);
	}

	fn cpy_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		let value: u8 = self.memory.read(address);
		self.compare(self.y, value);
		self.pc = self.pc.wrapping_add(2);
	}

	fn cmp_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		let value: u8 = self.memory.read(address);
		self.compare(self.a, value);
		self.pc = self.pc.wrapping_add(2);
	}

	fn dec_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		let value: u8 = self.memory.read(address);
		let result: u8 = value.wrapping_sub(1);
		self.memory.write(address, result);
		self.set_flags(result);
		self.pc = self.pc.wrapping_add(2);
	}

	fn bne(&mut self) {
		let offset: u8 = self.get_immediate();
		if !self.zero_flag {
			self.perform_relative_address(offset);
		} else {
			self.pc = self.pc.wrapping_add(1);
		}
	}

	fn cmp_indirect_y(&mut self) {
		let address: u16 = self.get_indirect_y();
		let value: u8 = self.memory.read(address);
		self.compare(self.a, value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn cmp_zeropage_x(&mut self) {
		let address: u16 = self.get_zeropage_x();
		let value: u8 = self.memory.read(address);
		self.compare(self.a, value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn dec_zeropage_x(&mut self) {
		let address: u16 = self.get_zeropage_x();
		let value: u8 = self.memory.read(address);
		let result: u8 = value.wrapping_sub(1);
		self.memory.write(address, result);
		self.set_flags(result);
		self.pc = self.pc.wrapping_add(1);
	}

	fn cld(&mut self) {
		self.decimal_flag = false;
	}

	fn cmp_absolute_y(&mut self) {
		let address: u16 = self.get_absolute_address_y();
		let value: u8 = self.memory.read(address);
		self.compare(self.a, value);
		self.pc = self.pc.wrapping_add(2);
	}

	fn cmp_absolute_x(&mut self) {
		let address: u16 = self.get_absolute_address_x();
		let value: u8 = self.memory.read(address);
		self.compare(self.a, value);
		self.pc = self.pc.wrapping_add(2);
	}

	fn dec_absolute_x(&mut self) {
		let address: u16 = self.get_absolute_address_x();
		let value: u8 = self.memory.read(address);
		let result: u8 = value.wrapping_sub(1);
		self.memory.write(address, result);
		self.set_flags(result);
		self.pc = self.pc.wrapping_add(2);
	}

	fn cpx_immediate(&mut self) {
		let value: u8 = self.get_immediate();
		self.compare(self.x, value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn sbc_indirect_x(&mut self) {
		let address: u16 = self.get_indirect_x();
		let value: u8 = self.memory.read(address);
		self.subtract_with_carry_decimal(value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn cpx_zeropage(&mut self) {
		let address: u16 = self.get_zeropage();
		let value: u8 = self.memory.read(address);
		self.compare(self.x, value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn sbc_zeropage(&mut self) {
		let address: u16 = self.get_zeropage();
		let value: u8 = self.memory.read(address);
		self.subtract_with_carry_decimal(value);
		self.pc = self.pc.wrapping_add(1);
	}
	

	fn inc_zeropage(&mut self) {
		let address: u16 = self.get_zeropage();
		let value: u8 = self.memory.read(address);
		let result: u8 = value.wrapping_add(1);
		self.memory.write(address, result);
		self.set_flags(result);
		self.pc = self.pc.wrapping_add(1);
	}

	fn inx(&mut self) {
		self.x = self.x.wrapping_add(1);
		self.set_flags(self.x);
	}

	fn sbc_immediate(&mut self) {
		let value: u8 = self.get_immediate();
		self.subtract_with_carry_decimal(value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn nop(&mut self) {
		// Nothing happens
	}

	fn cpx_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		let value: u8 = self.memory.read(address);
		self.compare(self.x, value);
		self.pc = self.pc.wrapping_add(2);
	}

	fn sbc_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		let value: u8 = self.memory.read(address);
		self.subtract_with_carry_decimal(value);
		self.pc = self.pc.wrapping_add(2);
	}

	fn inc_absolute(&mut self) {
		let address: u16 = self.get_absolute_address();
		let value: u8 = self.memory.read(address);
		let result: u8 = value.wrapping_add(1);
		self.memory.write(address, result);
		self.set_flags(result);
		self.pc = self.pc.wrapping_add(2);
	}



	fn sbc_indirect_y(&mut self) {
		let address: u16 = self.get_indirect_y();
		let value: u8 = self.memory.read(address);
		self.subtract_with_carry_decimal(value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn sbc_zeropage_x(&mut self) {
		let address: u16 = self.get_zeropage_x();
		let value: u8 = self.memory.read(address);
		self.subtract_with_carry_decimal(value);
		self.pc = self.pc.wrapping_add(1);
	}

	fn inc_zeropage_x(&mut self) {
		let address: u16 = self.get_zeropage_x();
		let value: u8 = self.memory.read(address);
		let result: u8 = value.wrapping_add(1);
		self.memory.write(address, result);
		self.set_flags(result);
		self.pc = self.pc.wrapping_add(1);
	}

	fn sed(&mut self) {
		self.decimal_flag = true;
	}

	fn sbc_absolute_y(&mut self) {
		let address: u16 = self.get_absolute_address_y();
		let value: u8 = self.memory.read(address);
		self.subtract_with_carry_decimal(value);
		self.pc = self.pc.wrapping_add(2);
	}

	fn sbc_absolute_x(&mut self) {
		let address: u16 = self.get_absolute_address_x();
		let value: u8 = self.memory.read(address);
		self.subtract_with_carry_decimal(value);
		self.pc = self.pc.wrapping_add(2);
	}

	fn inc_absolute_x(&mut self) {
		let address: u16 = self.get_absolute_address_x();
		let value: u8 = self.memory.read(address);
		let result: u8 = value.wrapping_add(1);
		self.memory.write(address, result);
		self.set_flags(result);
		self.pc = self.pc.wrapping_add(2);
	}

	fn adc(&mut self, n2: u8)
    {
        
		let c : u16 = if self.carry_flag { 1 } else { 0 };


        let value = n2 as u16;
        
        if !self.decimal_mode
        {
            let total = self.a as u16 + value + c;
            
            if total>255   {
                // Set the C flag
                self.carry_flag= true;
            }
            else   {
                // Clear the C flag
                self.carry_flag = false;
            }
            
            let operand0 = self.a & 0x80;
            let operand1 = n2 & 0x80;
            let result   = total & 0x80;
            
            if operand0 == 0 && operand1 == 0 && result != 0
            {
                self.overflow_flag = true;  // Set the V flag
            }
            else
            {
            
                if operand0 != 0 && operand1 != 0 && result == 0
                {
                    self.overflow_flag = true;
                }
                else
                {
                    self.overflow_flag = false;             // Clear the V flag
                }
            }
            
            self.a = (total & 0xFF) as u8;
            self.set_flags(self.a);
            
            return
            
        
        }
        else // decimal mode
        {
            self.adc_decimal(n2);
           
        }
        
    }
    
    fn adc_decimal(&mut self,s : u8)
    {
        // s = value to be added to accumulator
        
		let c : u8 = if self.carry_flag { 1 } else { 0 };
  
        // Lower nib
        let mut al = (self.a & 15) + (s & 15) + c;
        
        // Higher nib
        let mut ah = (self.a >> 4) + (s >> 4); if al > 9 { ah += 1 }
        
        // Wrap lower nib
        if al > 9 { al -= 10  }
        
        // Set Zero flag, but doesn't account for 0x80 answer yet
        //self.zero_flag = ((A &+ s &+ C) & 255 == 0) ? true : false

		if (self.a + s + c ) & 255 == 0 {self.carry_flag = true} else {self.carry_flag = false}
        
        self.negative_flag = ah & 8 != 0;
        self.overflow_flag = ((((ah << 4) ^ self.a) & 128) != 0) && !((((self.a ^ s) & 128) != 0));
        
        if ah > 9 { ah -= 10; self.carry_flag = true;} else {self.carry_flag = false;}
        
        // Calculate accumulator
        self.a = ((ah << 4) | (al & 15)) & 255;
        
        self.set_flags(self.a)
        
    }
    



}