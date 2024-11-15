use core::slice::Iter;
use std::{collections::HashSet, ops::{Index, IndexMut}};
use crate::register::Register;
use std::hash::Hash;



pub type Platter = u32;
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct MemoryAddress(Platter);
impl From<Platter> for MemoryAddress {
    fn from(n: Platter) -> Self {
        Self(n)
    }
}

pub type ArrayOfPlatters = Vec<Platter>;
type MemType = Vec<ArrayOfPlatters>;
type MemoryAddresses = Vec<MemoryAddress>;
pub struct Memory {
    mem: MemType,
    allocated: MemoryAddresses,
}

trait ToSet<T> {
    fn as_set(&self) -> HashSet<T>;
}

impl<T> ToSet<T> for Iter<'_,T> where T: Eq + Hash + Clone {
    fn as_set(&self) -> HashSet<T> {
        self.clone().cloned().collect()
    }
}

impl<T> ToSet<T> for [T] where T: Eq + Hash + Clone {
    fn as_set(&self) -> HashSet<T> {
        self.iter().cloned().collect()
    }
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
        let all = self.all_addresses();
        let allocated = self.allocated.as_set();
        let free = &all - &allocated;
        let addr = match free.iter().last() {
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
        };
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


    /* PRIVATE */
    fn all_addresses(&self) -> HashSet<MemoryAddress> {
        let mem_addrs: MemoryAddresses = (0..5).map(|x|x.into()).collect();
            .as_set()
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
