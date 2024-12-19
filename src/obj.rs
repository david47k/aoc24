#[repr(u8)]
#[derive(Clone,Copy,Eq,PartialEq,Hash,Debug)]
pub enum Obj {
    Space = 0,
    Wall = 1,
    Deer = 2,
}

impl Obj {
    pub fn from_char(c: &char) -> Obj {
        match c {
            '.' => Obj::Space,
            '#' => Obj::Wall,
            'd' => Obj::Deer,
            _ => panic ! ("Invalid character"),
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            Obj::Space => '.',
            Obj::Wall => '#',
            Obj::Deer => 'd',
        }
    }
}