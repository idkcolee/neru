const RAM_SIZE: usize = 0x2000;

struct Memory {
    ram: [u8; RAM_SIZE]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ram: [0x00; RAM_SIZE]
        }
    }
    
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram[address],
            _ => panic!("memory address out of range: {}", address)
        }
    }
    
    pub fn write(&mut self, value: u8, address: u16) {
        match address {
            0x0000..=0x1FFF => self.ram[address] = value,
            _ => panic!("memory address out of range: {}", address)
        }
    }
}