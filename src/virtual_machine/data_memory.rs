use super::control_unit::Mem;

pub type DataMemory = Vec<u8>;

pub trait DataMem {
    fn load(&self, addr: u32, size: Mem) -> Option<u32>;
    fn store(&mut self, data: u32, addr: u32, size: Mem);
}

impl DataMem for DataMemory {
    fn load(&self, addr: u32, size: Mem) -> Option<u32> {
        match size {
            Mem::None => None,
            Mem::Byte => self.get(addr as usize).map(|e| *e as u32),
            Mem::Half => Some(
                (0..2)
                    .map(|v| self.get(v).unwrap_or(&0))
                    .fold(0, |acc, e| acc + *e as u32),
            ),
            Mem::Word => Some(
                (0..4)
                    .map(|v| self.get(v).unwrap_or(&0))
                    .fold(0, |acc, e| acc + *e as u32),
            ),
        }
    }

    fn store(&mut self, data: u32, addr: u32, size: Mem) {
        let data = data.to_be_bytes();
        match size {
            Mem::None => {}
            Mem::Byte => {
                if let Some(elem) = self.get_mut(addr as usize) {
                    *elem = data[0];
                }
            }
            Mem::Half => {
                (0..2).for_each(|v| {
                    if let Some(elem) = self.get_mut(addr as usize + v) {
                        *elem = data[v];
                    }
                });
            }
            Mem::Word => {
                (0..4).for_each(|v| {
                    if let Some(elem) = self.get_mut(addr as usize + v) {
                        *elem = data[v];
                    }
                });
            }
        }
    }
}
