use std::{collections::HashSet, ops::{Index, IndexMut}};
use crate::register::Register;



pub type Platter = u32;
pub type ArrayOfPlatters = Vec<Platter>;
type MemType = Vec<ArrayOfPlatters>;
type AllocType = Vec<usize>;
pub struct Memory {
    mem: MemType,
    allocated: AllocType,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            mem: MemType::new(),
            allocated: AllocType::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.mem.len()
    }
    pub fn alloc(&mut self, size: usize) -> u32 {
        let all_addresses = HashSet::<usize>::from_iter(1..self.len());
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
                i
            }
        };
        self.allocated.push(addr);
        // println!("{} = alloc({})", addr, size);
        addr as u32
    }
    pub fn free(&mut self, addr: usize) {
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

impl Index<usize> for Memory {
    type Output = ArrayOfPlatters;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.mem[idx]
    }
}
impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.mem[idx]
    }
}
impl Index<Register> for Memory {
    type Output = ArrayOfPlatters;
    fn index(&self, idx: Register) -> &Self::Output {
        let i: usize = idx.into();
        &self.mem[i]
    }
}
impl IndexMut<Register> for Memory {
    fn index_mut(&mut self, idx: Register) -> &mut Self::Output {
        let i: usize = idx.into();
        &mut self.mem[i]
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
