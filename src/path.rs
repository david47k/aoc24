use crate::vector::{Vector, VectorSm};
use crate::path2::Move2;

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum Move { Up=1, Right=2, Down=4, Left=8 }

pub const DIR_U: u8 = 1;
pub const DIR_R: u8 = 2;
pub const DIR_D: u8 = 4;
pub const DIR_L: u8 = 8;
pub const ALLMOVES: [Move; 4] = [ Move::Up, Move::Right, Move::Down, Move::Left ];

impl Move {
    pub fn u(&self) -> u8 {
        *self as u8
    }
    pub fn rotr(&self) -> Move {
        match self {
            Move::Up    => Move::Right,
            Move::Right => Move::Down,
            Move::Down  => Move::Left,
            Move::Left  => Move::Up,
        }
    }
    pub fn rotl(&self) -> Move {
        match self {
            Move::Up    => Move::Left,
            Move::Left  => Move::Down,
            Move::Down  => Move::Right,
            Move::Right => Move::Up,
        }
    }
    pub fn to_vector(&self) -> Vector {
        match self {
            Move::Up    => Vector( 0, -1 ),
            Move::Right => Vector( 1,  0 ),
            Move::Down  => Vector( 0,  1 ),
            Move::Left  => Vector(-1,  0 ),
        }
    }
    pub fn from_u8_unchecked(n: u8) -> Move {
        if n==1 { Move::Up }
        else if n==2 { Move::Right }
        else if n==4 { Move::Down }
        else if n==8 { Move::Left }
        else { panic!("unexpected move value"); }
    }
    pub fn from_u8(n: u8) -> Option<Move> {
        match n {
            1 => Some(Move::Up),
            2 => Some(Move::Right),
            4 => Some(Move::Down),
            8 => Some(Move::Left),
            _ => None,
        }
    }
    pub fn reverse(&self) -> Move {
        match self {
            Move::Up	=> Move::Down,
            Move::Left	=> Move::Right,
            Move::Right	=> Move::Left,
            Move::Down	=> Move::Up,
        }
    }
    pub fn from_char_unchecked(c: char) -> Move {
        match c {
            '^'	=> Move::Up,
            '>' => Move::Right,
            'v' => Move::Down,
            '<' => Move::Left,
            _   => panic!("unexpected move value"),
        }
    }
    pub fn to_vector_sm(&self) -> VectorSm {
        match self {
            Move::Up    => VectorSm( 0, -1 ),
            Move::Right => VectorSm( 1,  0 ),
            Move::Down  => VectorSm( 0,  1 ),
            Move::Left  => VectorSm(-1,  0 ),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Move::Up    => String::from("U"),
            Move::Right => String::from("R"),
            Move::Down  => String::from("D"),
            Move::Left  => String::from("L"),
        }
    }
    pub fn as_move2(&self) -> Move2 {
        match self {
            Move::Up    => Move2::Up,
            Move::Right => Move2::Right,
            Move::Down  => Move2::Down,
            Move::Left  => Move2::Left,
        }
    }

}



pub struct BasicPath {
    data: Vec<Move>,
}
impl BasicPath {
    fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }
    fn clear(&mut self) {
        self.data.clear();
    }
    fn len(&self) -> u16 {
        self.data.len().try_into().unwrap()
    }
    fn from_path(path: &Vec::<Move>) -> Self {
        Self {
            data: path.clone(),
        }
    }
    fn push(&mut self, move1: &Move) {
        self.data.push(move1.clone());
    }
    fn append_path(&mut self, path: &Vec::<Move>) {
        self.data.append(&mut path.clone());
    }
    fn to_path(&self) -> Vec::<Move> {
        self.data.clone()
    }
    fn score(&self) -> u64 {
        // actual movement scores 1 point
        // a rotation by 90 degrees scores 1000 points
        let mut score: u64 = 0;
        let path = &self.data;

        for idx in 0..self.len() as usize {
            let pair = if idx > 0 {
                [ path[idx-1], path[idx] ]
            } else {
                [ Move::Right, path[idx] ]
            };
            if pair[0] == pair[1] { // only scores 1 for movement
                score += 1;
            } else if pair[0] == pair[1].rotr() || pair[0] == pair[1].rotl() {
                score += 1001;
            } else if pair[0] == pair[1].rotr().rotr() {
                score += 2001;  // terrible choice, making a u-turn!
            } else {
                panic!("shouldn't get here");
            }
        }
        score
    }
    fn to_string(&self) -> String {
        let mut s: String = "".to_string();
        for m in self.data.iter() {
            s = s + &m.to_string();
        }
        return s;
    }
}

