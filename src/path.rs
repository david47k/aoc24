use crate::vector::{Vector, VectorSm};
use crate::path2::Move2;

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum Move { Up=1, Right=2, Down=4, Left=8 }

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

