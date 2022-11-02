// 64Kb of RAM for the 6502

// This is 64Kb of memory. It has some special addresses that are for the various
// 6502-based machines that are emulated. For example, the Apple-1 has specific addresses
// for character output and keyboard input.


// An 8bit memory cell with a flag to mark ROM
#[derive(Debug, Copy, Clone)]
 struct MemoryCell {
    pub value : u8,
    pub readonly : bool,
}

//Define 64Kb of memory
#[derive(Debug, Copy, Clone)]
pub struct MemoryArray {
     memory : [MemoryCell; 65536],
	 pub apple_output_char_waiting : bool,
	 pub apple_output_char : u8,
	 pub apple_key_ready : bool,
	 pub apple_key_value : u8,
}

impl MemoryArray {

    fn new() -> MemoryArray {
        MemoryArray { memory: [MemoryCell { value: 0, readonly: false }; 65536] ,
		apple_output_char_waiting : false, apple_output_char : 0,
		 apple_key_ready : false, apple_key_value : 0} // Zero it.
    }

    pub fn init() -> MemoryArray {
        let memory_map : MemoryArray = MemoryArray::new();
        memory_map
   }

   // The hard-working 'give me a byte at this address' function
   pub fn read(&mut self, address : u16) -> u8 {
	
	// Apple specific keyboard input

	if address == 0xd012  ||  address == 0xD0F2 {
		return 0x00
	}

	if address == 0xd010 {

		//println!("Key pressed: {}",self.apple_key_value);

		if self.apple_key_value == 92 {
			self.apple_key_value = 10
		}

		if self.apple_key_value >= 0x61 && self.apple_key_value <= 0x7A
		{
			self.apple_key_value = self.apple_key_value & 0x5f;
		}
			if self.apple_key_value == 10
			{
				self.apple_key_value = 13;
			}

			self.apple_key_ready = false;

			//println!("Key pressed: {:02X}",self.apple_key_value | 0x80);
			
			return self.apple_key_value | 0x80;
	}
	

	if address == 0xd011 {
		if self.apple_key_ready {
			return 0x80
		} else {
			return 0x00
		}
	}

       self.memory[address as usize].value
   }

   // Matching 'set a bytes at this address' function
   pub fn write(&mut self, address : u16, value : u8) {

        // Apple WozMon print a character to the screen
        if address == 0xd012 {
			self.apple_output_char_waiting = true;
			self.apple_output_char = value & 0x7f;
            print!("{}", (self.apple_output_char) as char);

			if self.apple_output_char == 10 || self.apple_output_char == 13  {
				print!("\n");
			}
        }

       if !self.memory[address as usize].readonly {
           self.memory[address as usize].value = value;
       }
   }

}