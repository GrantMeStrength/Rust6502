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
	 pub apple_key_ready : bool,
	 pub apple_key_char : u8,
}

impl MemoryArray {

    fn new() -> MemoryArray {
        MemoryArray { memory: [MemoryCell { value: 0, readonly: false }; 65536] , apple_key_ready : false, apple_key_char : 0} // Zero it.
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
		self.apple_key_ready = false;
		return self.apple_key_char | 0x80;
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
            print!("{}", (value & 0x7f) as char);
        }

       if !self.memory[address as usize].readonly {
           self.memory[address as usize].value = value;
       }
   }

}