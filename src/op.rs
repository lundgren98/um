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

impl Into<u32> for Op {
    fn into(self) -> u32 {
        match self {
            Self::Move => 0,
            Self::Index => 1,
            Self::Amend => 2,
            Self::Add => 3,
            Self::Mult => 4,
            Self::Div => 5,
            Self::NotAnd => 6,
            Self::Halt => 7,
            Self::Alloc => 8,
            Self::Aband => 9,
            Self::Output => 10,
            Self::Input => 11,
            Self::Load => 12,
            Self::Orth => 13,
        }
    }
}
