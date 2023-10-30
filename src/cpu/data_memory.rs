use super::{control_unit::Mem, DATA_MEM_START};

pub type DataMemory = Vec<u8>;

pub trait DataMem {
    fn load(&self, addr: u32, size: Mem) -> Option<u32>;
    fn store(&mut self, data: u32, addr: u32, size: Mem) -> Result<(), ()>;
}

impl DataMem for DataMemory {
    fn load(&self, addr: u32, size: Mem) -> Option<u32> {
        let offset_addr = addr.wrapping_sub(DATA_MEM_START);
        match size {
            Mem::None => None,
            Mem::Byte => self.get(offset_addr as usize).map(|e| *e as u32),
            Mem::Half => Some(
                (0..2)
                    .map(|v| self.get(offset_addr as usize + v).unwrap_or(&0))
                    .fold(0, |acc, e| acc + *e as u32),
            ),
            Mem::Word => Some(
                (0..4)
                    .map(|v| self.get(offset_addr as usize + v).unwrap_or(&0))
                    .fold(0, |acc, e| acc + *e as u32),
            ),
        }
    }

    fn store(&mut self, data: u32, addr: u32, size: Mem) -> Result<(), ()> {
        let offset_addr = addr.wrapping_sub(DATA_MEM_START);
        let data = data.to_be_bytes();
        match size {
            Mem::None => Ok(()),
            Mem::Byte => {
                if let Some(elem) = self.get_mut(offset_addr as usize) {
                    *elem = data[0];
                    Ok(())
                } else {
                    Err(())
                }
            }
            Mem::Half => (0..2)
                .try_for_each(|v| {
                    if let Some(elem) = self.get_mut(offset_addr as usize + v) {
                        *elem = data[v];
                        Ok(())
                    } else {
                        Err(())
                    }
                }),
            Mem::Word => (0..4)
                .try_for_each(|v| {
                    if let Some(elem) = self.get_mut(offset_addr as usize + v) {
                        *elem = data[v];
                        Ok(())
                    } else {
                        Err(())
                    }
                }),
        }
    }
}
