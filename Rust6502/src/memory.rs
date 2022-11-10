// 64Kb of RAM for the 6502

// This is 64Kb of memory. It has some special addresses that are for the various
// 6502-based machines that are emulated. For example, the Apple-1 has specific addresses
// for character output and keyboard input, hurrah for the 6502's memory mapped I/O! ;-)

// An 8bit memory cell with a flag to mark ROM
#[derive(Debug, Copy, Clone)]
struct MemoryCell {
    pub value: u8,
    pub readonly: bool,
}

//Define 64Kb of memory
#[derive(Debug, Copy, Clone)]
pub struct MemoryArray {
    memory: [MemoryCell; 65536],
    pub apple_output_char_waiting: bool,
    pub apple_output_char: u8,
    pub apple_key_ready: bool,
    pub apple_key_value: u8,
}

impl MemoryArray {
    fn new() -> MemoryArray {
        MemoryArray {
            memory: [MemoryCell {
                value: 0,
                readonly: false,
            }; 65536],
            apple_output_char_waiting: false,
            apple_output_char: 0,
            apple_key_ready: false,
            apple_key_value: 0,
        } // Zero it.
    }

    pub fn init() -> MemoryArray {
        let memory_map: MemoryArray = MemoryArray::new();
        memory_map
    }

    // The hard-working 'give me a byte at this address' function.
    // It has some extra stuff to handle the Apple-1's I/O.

    pub fn read(&mut self, address: u16) -> u8 {
        // Apple specific keyboard input

        if address == 0xD012 || address == 0xD0F2 {
            return 0x00;
        }

        // Check to see if there is a keypress for us to process.
        if address == 0xD010 {
            if self.apple_key_value == 92 {
                self.apple_key_value = 10
            }

            if self.apple_key_value >= 0x61 && self.apple_key_value <= 0x7A {
                self.apple_key_value &= 0x5f;
            }

            if self.apple_key_value == 10 {
                self.apple_key_value = 13;
            }
            self.apple_key_ready = false;
            return self.apple_key_value | 0x80;
        }

        if address == 0xd011 {
            if self.apple_key_ready {
                return 0x80;
            } else {
                return 0x00;
            }
        }

        // Nothing special, return memory contents
        self.memory[address as usize].value
    }

    pub fn write_with_status(&mut self, address: u16, value: u8, ro: bool) {
        self.memory[address as usize].value = value;
        self.memory[address as usize].readonly = ro;
    }

    // The 'set a byte at this address' function, with
    // some extra stuff for the Apple 1 character display code.
    pub fn write(&mut self, address: u16, value: u8) {
        
        // If this isn't ROM, then write to it.
        // The WozMon and Apple BASIC memory is marked read-only.
        if !self.memory[address as usize].readonly {
            self.memory[address as usize].value = value;
        }
    }
}
