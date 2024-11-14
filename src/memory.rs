use std::{collections::HashSet, ops::{Index, IndexMut}};
use crate::register::Register;



pub type Platter = u32;
pub type MemoryAddress = Platter;
pub type ArrayOfPlatters = Vec<Platter>;
type MemType = Vec<ArrayOfPlatters>;
type MemoryAddresses = Vec<MemoryAddress>;
pub struct Memory {
    mem: MemType,
    allocated: MemoryAddresses,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            mem: MemType::new(),
            allocated: MemoryAddresses::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.mem.len()
    }
    pub fn alloc(&mut self, size: usize) -> MemoryAddress {
        let all_addresses = HashSet::<MemoryAddress>::from_iter(1..self.len() as MemoryAddress);
        let free_addresses = &all_addresses - &(self.allocated.iter().cloned().collect());
        let free_addr = free_addresses.iter().find(|_| true); // Just give me an element
        let addr = match free_addr {
            None => {
                let len = self.len();
                if len > u32::MAX as usize {
                    panic!("Trying to allocate more memory than the machine is allowed to have.");
                }
                let v = vec![0u32; size];
                self.mem.push(v);
                len
            }
            Some(&i) => {
                self[i].resize(size, 0);
                i as usize
            }
        } as MemoryAddress;
        self.allocated.push(addr);
        // println!("{} = alloc({})", addr, size);
        addr as u32
    }
    pub fn free(&mut self, addr: MemoryAddress) {
        // println!("free({})", addr);
        if addr == 0 {
            panic!("Cannot abandon array 0");
        }
        match self.allocated.iter().enumerate().find(|(_, &a)| a == addr) {
            None => panic!("Cannot abandon unallocated array {}", addr),
            Some((i, _)) => {
                self.allocated.remove(i);
            }
        }
        self[addr].resize(0, 0);
        assert_eq!(self[addr].len(), 0);
    }
}

impl Index<MemoryAddress> for Memory {
    type Output = ArrayOfPlatters;
    fn index(&self, idx: MemoryAddress) -> &Self::Output {
        let i = idx as usize;
        &self.mem[i]
    }
}
impl IndexMut<MemoryAddress> for Memory {
    fn index_mut(&mut self, idx: MemoryAddress) -> &mut Self::Output {
        let i = idx as usize;
        &mut self.mem[i]
    }
}
impl Index<Register> for Memory {
    type Output = ArrayOfPlatters;
    fn index(&self, idx: Register) -> &Self::Output {
        let i: MemoryAddress = idx.into();
        &self[i]
    }
}
impl IndexMut<Register> for Memory {
    fn index_mut(&mut self, idx: Register) -> &mut Self::Output {
        let i: MemoryAddress = idx.into();
        &mut self[i]
    }
}
impl Index<Register> for ArrayOfPlatters {
    type Output = u32;
    fn index(&self, idx: Register) -> &Self::Output {
        let i: usize = idx.into();
        &self[i]
    }
}
impl IndexMut<Register> for ArrayOfPlatters {
    fn index_mut(&mut self, idx: Register) -> &mut Self::Output {
        let i: usize = idx.into();
        &mut self[i]
    }
}
