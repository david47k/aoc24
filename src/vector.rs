// vector.rs, extracted from Box Bopper: Sokoban-like game
// Copyright David Atkinson 2020-2021
//
// vector.rs: has vector for points / moves / directions and paths
//
// A point and a direction can both be implemented as a Vector

use crate::grid::Grid;
use crate::path::Move;
use crate::path2::Move2;

#[derive(Clone, Copy, PartialEq, Ord, PartialOrd, Eq, Debug)]
pub struct Vector (pub i32, pub i32);

impl Vector {
    pub fn new(x: i32, y: i32) -> Vector {
        Self(x,y)
    }
    pub fn add(&self, dir: &Vector) -> Self {
        Self(self.0 + dir.0, self.1 + dir.1)
    }
    pub fn sub(&self, v: &Vector) -> Self {
        Self(self.0 - v.0, self.1 - v.1)
    }
    pub fn distance(&self, v: &Vector) -> i32 {
        (self.0 - v.0).abs() + (self.1 - v.1).abs()
    }
    pub fn double(&self) -> Self {
        Self(self.0 * 2, self.1 * 2)
    }
    pub fn mul(&self, n: i32) -> Self {
        Self(self.0 * n, self.1 * n)
    }
    pub fn rotr(&self) -> Self {
        Self(self.1, -self.0)
    }
    pub fn rotl(&self) -> Self {
        Self(-self.1, self.0)
    }
    pub fn scale_by(&self, n: i32) -> Self {
        Self(self.0 * n, self.1 * n)
    }
    pub fn eq(&self, a: &Vector) -> bool {
        self.0 == a.0 && self.1 == a.1
    }
    pub fn add_dir(&self, dir: &Move) -> Self {
        match dir {
            Move::Up    => Self( self.0,   self.1-1 ),
            Move::Right => Self( self.0+1, self.1   ),
            Move::Down  => Self( self.0,   self.1+1 ),
            Move::Left  => Self( self.0-1, self.1   ),
        }
    }
    pub fn apply_dir(&self, dir: &Move2) -> Self {
        match dir {
            Move2::Up    => Self( self.0,   self.1-1 ),
            Move2::Right => Self( self.0+1, self.1   ),
            Move2::Down  => Self( self.0,   self.1+1 ),
            Move2::Left  => Self( self.0-1, self.1   ),
        }
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


#[derive(Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
pub struct VectorSm ( pub i8, pub i8 );
impl VectorSm {
    pub fn fromv(v: &Vector) -> Self {
        Self ( v.0 as i8, v.1 as i8 )
    }
    pub fn intov(&self) -> Vector {
        Vector(self.0 as i32, self.1 as i32)
    }
    pub fn new(x: i8, y: i8) -> Self {
        Self (x,y)
    }
    pub fn add(&self, dir: &Self) -> Self {
        Self (self.0 + dir.0, self.1 + dir.1)
    }
    pub fn double(&self) -> Self {
        Self (self.0 * 2, self.1 * 2)
    }
    pub fn mul(&self, n: i8) -> Self {
        Self(self.0 * n, self.1 * n)
    }
    pub fn rotr(&self) -> Self {
        Self(self.1, -self.0)
    }
    pub fn rotl(&self) -> Self {
        Self(-self.1, self.0)
    }
    pub fn add_dir(&self, dir: &Move) -> Self {
        match dir {
            Move::Up    => Self( self.0,   self.1-1 ),
            Move::Right => Self( self.0+1, self.1   ),
            Move::Down  => Self( self.0,   self.1+1 ),
            Move::Left  => Self( self.0-1, self.1   ),
        }
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
}




// SuperShrunkPath stores part of the path as an index to existing path strings... thus allowing us to shorten the path data due to reuse... it'll use a lot more cpu though, to find matching path strings
// Once we get more than 64 moves, we can store the original moves as a 'prefix' index... 64 moves is 128 bits, plus we get reduction in mem usage as there will be lots of repeats

pub struct SuperPrefix {
    data: Vec<u128>,
}

impl SuperPrefix {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }
    pub fn get_by_index(&mut self, i: u32) -> Option<u128> {
        if i as usize >= self.len() {
            return None;
        }
        return Some(self.data[i as usize]);
    }
    pub fn add_unchecked(&mut self, d: u128) -> u32 {
        let i = self.data.len();
        self.data.insert(i, d);		// thread safe version, but can push data around a bit
        return i as u32;			// not properly checked
    }
    pub fn add(&mut self, d: u128) -> u32 {
        // basic linear search ugh
        for idx in 0..self.data.len() {
            if self.data[idx] == d {
                return idx as u32;
            }
        }

        // we couldn't find it, add it
        let i = self.data.len();
        self.data.insert(i, d);		// thread safe version, but can push data around a bit
        return i as u32;			// not properly checked
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

