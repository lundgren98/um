#[derive(Debug)]
pub enum Op {
    Move,
    Index,
    Amend,
    Add,
    Mult,
    Div,
    NotAnd,
    Halt,
    Alloc,
    Aband,
    Output,
    Input,
    Load,
    Orth,
}

impl From<u8> for Op {
    fn from(op: u8) -> Self {
        match op {
            0 => Self::Move,
            1 => Self::Index,
            2 => Self::Amend,
            3 => Self::Add,
            4 => Self::Mult,
            5 => Self::Div,
            6 => Self::NotAnd,
            7 => Self::Halt,
            8 => Self::Alloc,
            9 => Self::Aband,
            10 => Self::Output,
            11 => Self::Input,
            12 => Self::Load,
            13 => Self::Orth,
            _ => panic!(),
        }
    }
}
