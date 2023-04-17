const MEM_SIZE: usize = 0x0800; 

struct Memory {
    data: [u8; MEM_SIZE]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            data: [0x00; MEM_SIZE]
        }
    }
    
    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }
    
    pub fn write(&mut self, value: u8, address: u16) {
        self.data[address as usize] = value;
    }
}