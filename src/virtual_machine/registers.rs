pub type Registers = [u32; 32];

pub trait Register {
    fn read(&self, addr: u32) -> u32;
    fn write(&mut self, data: u32, addr: u32, write: bool);
}

impl Register for Registers {
    fn read(&self, addr: u32) -> u32 {
        self[addr as usize]
    }

    fn write(&mut self, data: u32, addr: u32, write: bool) {
        if write {
            self[addr as usize] = data;
        }
    }
}
