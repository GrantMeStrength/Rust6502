// 64Kb of RAM for the 6502

// Define 8bit memory cell
#[derive(Debug, Copy, Clone)]
 struct MemoryCell {
    pub value : u8,
    pub readonly : bool,
}

//Define 64Kb of memory
#[derive(Debug, Copy, Clone)]
pub struct MemoryArray {
     memory : [MemoryCell; 65536],
}

impl MemoryArray {
    fn new() -> MemoryArray {
        MemoryArray { memory: [MemoryCell { value: 0, readonly: false }; 65536] }
    }

    pub fn init() -> MemoryArray {
        let mut memory_map : MemoryArray = MemoryArray::new();
        memory_map
 
   }

   pub fn read(&self, address : u16) -> u8 {
       self.memory[address as usize].value
   }

   pub fn write(&mut self, address : u16, value : u8) {
       if !self.memory[address as usize].readonly {
           self.memory[address as usize].value = value;
       }
   }

}