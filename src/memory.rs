use core::slice::Iter;
use std::{collections::HashSet, ops::{Index, IndexMut}};
use crate::{macros::{impl_from, impl_index, impl_into, impl_into_via}, register::Register};
use std::hash::Hash;


pub type Platter = u32;
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct MemoryAddress(Platter);
impl_from!(MemoryAddress, Platter);
impl_into!(MemoryAddress, Platter);
impl_into_via!(MemoryAddress, Platter, Register);

#[derive(Debug, PartialEq, Clone)]
pub struct Collection<T>(Vec<T>);
impl<T, U> From<Vec<U>> for Collection<T> where T: From<U>, U: Copy {
    fn from(u: Vec<U>) -> Collection<T> {
        let v: Vec<T> = u.iter().map(|&x| Into::<T>::into(x)).collect();
        Self(v)
    }
}
impl<T, A> FromIterator<A> for Collection<T> where Vec<T>: FromIterator<A> {
    fn from_iter<S: IntoIterator<Item = A>>(iter: S) -> Self {
        let v: Vec<T> = iter.into_iter().collect();
        Self(v)
    }
}

type Array<T> = Collection<T>;
pub type ArrayOfPlatters = Array<Platter>;
/* TODO: Maybe have MemType be a HashMap<MemoryAddress, ArrayOfPlatters> ?*/
type MemType = Collection<ArrayOfPlatters>;
impl_index!(MemType, MemoryAddress, ArrayOfPlatters);
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
                let v: ArrayOfPlatters = vec![0u32; size].into();
                self.mem.push(v);
                (len as u32).into()
            }
            Some(&i) => {
                self[i].resize(size, 0);
                i
            }
        };
        self.allocated.push(addr);
        // println!("{} = alloc({})", addr, size);
        addr
    }
    pub fn free(&mut self, addr: MemoryAddress) {
        // println!("free({})", addr);
        if addr == 0.into() {
            panic!("Cannot abandon array 0");
        }
        match self.allocated.iter().enumerate().find(|(_, &a)| a == addr) {
            None => panic!("Cannot abandon unallocated array {}", Into::<Platter>::into(addr)),
            Some((i, _)) => {
                self.allocated.remove(i);
            }
        }
        self[addr].resize(0, 0);
        assert_eq!(self[addr].len(), 0);
    }


    /* PRIVATE */
    fn all_addresses(&self) -> HashSet<MemoryAddress> {
        (0..5).map(|x|x.into()).collect::<MemoryAddresses>().as_set()
    }
}


impl<T> Index<usize> for Collection<T> {
    type Output = T;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}
impl<T> IndexMut<usize> for Collection<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}
impl<T> Collection<T> where T: Clone {
    fn resize(&mut self, new_len: usize, value: T) {
        let v = &mut self.0;
        v.resize(new_len, value.into());
    }
}
impl<T> Collection<T> {
    fn len(&self) -> usize {
        self.0.len()
    }
    fn push(&mut self, t: T) {
        self.0.push(t)
    }
}

impl Into<usize> for MemoryAddress {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl From<Register> for MemoryAddress {
    fn from(value: Register) -> Self {
        let p: Platter = value.into();
        p.into()
    }
}

impl Index<MemoryAddress> for Memory {
    type Output = ArrayOfPlatters;
    fn index(&self, idx: MemoryAddress) -> &Self::Output {
        let i: Platter = idx.into();
        &self.mem[i as usize]
    }
}
impl IndexMut<MemoryAddress> for Memory {
    fn index_mut(&mut self, idx: MemoryAddress) -> &mut Self::Output {
        let i: Platter = idx.into();
        &mut self.mem[i as usize]
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


impl<T> Collection<T> {
    fn new() -> Self {
        Self(Vec::new())
    }
}
