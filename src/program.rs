use crate::memory::{ArrayOfPlatters, Platter};

pub type Source = Vec<u8>;
const PLATTER_SIZE: usize = 4;
pub type Partition = [u8; PLATTER_SIZE];

fn parition(source: Source) -> Vec<Partition> {
    (0..source.len() / PLATTER_SIZE).map(|i| {
        let start = PLATTER_SIZE * i;
        let end = PLATTER_SIZE * (i + 1);
        let range = start..end;
        let p: Partition = source[range].try_into().unwrap();
        p
    }).collect()
}

fn big_endian(p: Partition) -> Platter {
    p.map(|x|x as Platter)
        .iter()
        .rev()
        .enumerate()
        .map(|(i,x): (usize, &Platter)| x << (i * 8))
        .sum()
}

type ProgramType = ArrayOfPlatters;
#[derive(Debug)]
pub struct Program(ProgramType);
impl From<Source> for Program {
    fn from(source: Source) -> Self {
        parition(source)
            .iter()
            .map(|&p| big_endian(p))
            .collect::<ProgramType>()
            .into()
    }
}

impl PartialEq<ProgramType> for Program {
    fn eq(&self, other: &ProgramType) -> bool {
        self.0 == *other
    }
}

impl From<ProgramType> for Program {
    fn from(v: ProgramType) -> Self {
        Self(v)
    }
}

impl Into<ProgramType> for Program {
    fn into(self) -> ProgramType {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::memory::Collection;
    use super::*;

    #[test]
    fn parse_program() {
        let source: Vec<u8> = vec![0xde, 0xad, 0xbe, 0xef, 0xba, 0xbe, 0xca, 0xfe];
        let got: Program = source.into();
        let tmp: Collection<Platter> = vec![0xdeadbeefu32, 0xbabecafeu32].into();
        let expected: Program = tmp.into();
        assert_eq!(format!("{:x?}", got), format!("{:x?}", expected));
    }
}

