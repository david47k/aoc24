// vector.rs, extracted from Box Bopper: Sokoban-like game
// Copyright David Atkinson 2020-2021
//
// vector.rs: has vector for points / moves / directions and paths
//
// A point and a direction can both be implemented as a Vector

use crate::grid::Grid;

#[derive(Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
pub struct Vector (pub isize, pub isize);

impl Vector {
    pub fn new(x: isize, y: isize) -> Vector {
        Self(x,y)
    }
    pub fn add(&self, dir: &Vector) -> Self {
        Self(self.0 + dir.0, self.1 + dir.1)
    }
    pub fn sub(&self, v: &Vector) -> Self {
        Self(self.0 - v.0, self.1 - v.1)
    }
    pub fn double(&self) -> Self {
        Self(self.0 * 2, self.1 * 2)
    }
    pub fn mul(&self, n: isize) -> Self {
        Self(self.0 * n, self.1 * n)
    }
    pub fn rotr(&self) -> Self {
        Self(self.1, -self.0)
    }
    pub fn rotl(&self) -> Self {
        Self(-self.1, self.0)
    }
    pub fn scale_by(&self, n: isize) -> Self {
        Self(self.0 * n, self.1 * n)
    }
    pub fn eq(&self, a: &Vector) -> bool {
        self.0 == a.0 && self.1 == a.1
    }
}

// non-js
impl Vector {
    pub fn add_dir(&self, dir: &Move) -> Self {
        let d = *dir as i32; //1, 2, 4, 8
        Self(self.0+((d==1) as isize)-((d==3) as isize),self.1-((d==0) as isize)+((d==2) as isize))
        /*		match dir {
                    Move::Up    => Self( self.0,   self.1-1 ),
                    Move::Right => Self( self.0+1, self.1   ),
                    Move::Down  => Self( self.0,   self.1+1 ),
                    Move::Left  => Self( self.0-1, self.1   ),
                }		*/
    }
    pub fn add_dir2(&self, dir: &Move) -> Self {
        match dir {
            Move::Up    => Self( self.0,   self.1-2 ),
            Move::Right => Self( self.0+2, self.1   ),
            Move::Down  => Self( self.0,   self.1+2 ),
            Move::Left  => Self( self.0-2, self.1   ),
        }
    }
    pub fn to_index(&self, width: u16) -> usize {
        width as usize * (self.1 as usize) + (self.0 as usize)
    }
    pub fn to_usize(&self) -> (usize,usize) {
        (self.0 as usize, self.1 as usize)
    }
    pub fn to_string(&self) -> String {
        format!("({},{})",self.0,self.1)
    }
    pub fn is_valid(&self, grid: &Grid) -> bool {
        self.0 >= 0 && self.0 < grid.w && self.1 >= 0 && self.1 < grid.h
    }
}


#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Move { Up=1, Right=2, Down=4, Left=8 }
pub const DIR_U: u8 = 1;
pub const DIR_R: u8 = 2;
pub const DIR_D: u8 = 4;
pub const DIR_L: u8 = 8;

impl Move {
    pub fn u(&self) -> u8 {
        *self as u8
    }
    pub fn to_xy(&self) -> Vector {
        match self {
            Move::Up    => Vector::new( 0, -1 ),
            Move::Right => Vector::new( 1,  0 ),
            Move::Down  => Vector::new( 0,  1 ),
            Move::Left  => Vector::new(-1,  0 ),
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
    pub fn to_string(&self) -> String {
        match self {
            Move::Up    => String::from("U"),
            Move::Right => String::from("R"),
            Move::Down  => String::from("D"),
            Move::Left  => String::from("L"),
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
}

pub const ALLMOVES: [Move; 4] = [ Move::Up, Move::Right, Move::Down, Move::Left ];
