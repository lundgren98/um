/* sizes in bits */
const PLATTER_SIZE: u32 = 32;
const OP_SIZE: u32 = 4;
/* offsets */
const OP_OFFSET: u32 = PLATTER_SIZE - OP_SIZE;

pub fn align(input: Vec<u8>) -> Vec<u32> {
    let aligned: Vec<u32> = input
        .into_iter()
        .enumerate()
        .map(|(i, c)| {
            let offset = ((3 - (i % 4)) % 4) * 8;
            assert!(offset < OP_OFFSET as usize);
            let aligned = (c as u32) << offset;
            aligned
        })
        .collect();
    aligned
}

pub fn fuse(aligned: Vec<u32>) -> Vec<u32> {
    let program: Vec<u32> = (0..aligned.len() / 4)
        .map(|i| {
            let fourth = i * 4;
            aligned[fourth..fourth + 4].into_iter().sum()
        })
        .collect();
    program
}

type ProgramType = Vec<u32>;
#[derive(Debug)]
pub struct Program(ProgramType);
impl From<Vec<u8>> for Program {
    fn from(input: Vec<u8>) -> Self {
        let aligned = align(input);
        let program = fuse(aligned);
        program.into()
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
    use super::*;

    #[test]
    fn test_align() {
        let source: Vec<u8> = vec![0xde, 0xad, 0xbe, 0xef, 0xba, 0xbe, 0xca, 0xfe];
        let got = align(source);
        let expected = vec![
            0xde000000, 0x00ad0000, 0x0000be00, 0x000000ef, 0xba000000, 0x00be0000, 0x0000ca00,
            0x000000fe,
        ];
        assert_eq!(got, expected);
    }

    #[test]
    fn test_fuse() {
        let nums: Vec<u32> = vec![
            0xde000000, 0x00ad0000, 0x0000be00, 0x000000ef, 0xba000000, 0x00be0000, 0x0000ca00,
            0x000000fe,
        ];
        let got = fuse(nums);
        let expected: Vec<u32> = vec![0xdeadbeef, 0xbabecafe];
        assert_eq!(got, expected);
    }

    #[test]
    fn parse_program() {
        let source: Vec<u8> = vec![0xde, 0xad, 0xbe, 0xef, 0xba, 0xbe, 0xca, 0xfe];
        let got: Program = source.into();
        let expected: Vec<u32> = vec![0xdeadbeef, 0xbabecafe];
        assert_eq!(got, expected);
    }
}

