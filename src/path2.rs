use std::cmp::PartialEq;
use crate::path::Move;
use crate::stackstack::StackStack64;
use crate::vector::{Vector, VectorSm};

#[derive(PartialOrd, Eq, Ord, Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum Move2 { Up=0, Right=1, Down=2, Left=3 }
pub const DIR2_U: u8 = 0;
pub const DIR2_R: u8 = 1;
pub const DIR2_D: u8 = 2;
pub const DIR2_L: u8 = 3;

pub const ALLMOVES2: [Move2; 4] = [ Move2::Up, Move2::Right, Move2::Down, Move2::Left ];


impl Move2 {
    pub fn to_vector(&self) -> Vector {
        match self {
            Move2::Up    => Vector( 0, -1 ),
            Move2::Right => Vector( 1,  0 ),
            Move2::Down  => Vector( 0,  1 ),
            Move2::Left  => Vector(-1,  0 ),
        }
    }
    pub fn from_u8_unchecked(n: u8) -> Move2 {
        if n==0 { Move2::Up }
        else if n==1 { Move2::Right }
        else if n==2 { Move2::Down }
        else if n==3 { Move2::Left }
        else { panic!("unexpected move value"); }
    }
    pub fn from_u8(n: u8) -> Option<Move2> {
        match n {
            0 => Some(Move2::Up),
            1 => Some(Move2::Right),
            2 => Some(Move2::Down),
            3 => Some(Move2::Left),
            _ => None,
        }
    }
    pub fn to_vector_sm(&self) -> VectorSm {
        match self {
            Move2::Up    => VectorSm( 0, -1 ),
            Move2::Right => VectorSm( 1,  0 ),
            Move2::Down  => VectorSm( 0,  1 ),
            Move2::Left  => VectorSm(-1,  0 ),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Move2::Up    => String::from("U"),
            Move2::Right => String::from("R"),
            Move2::Down  => String::from("D"),
            Move2::Left  => String::from("L"),
        }
    }
    pub fn as_move(&self) -> Move {
        match self {
            Move2::Up    => Move::Up,
            Move2::Right => Move::Right,
            Move2::Down  => Move::Down,
            Move2::Left  => Move::Left,
        }
    }
    pub fn rotr(&self) -> Move2 {
        match self {
            Move2::Up    => Move2::Right,
            Move2::Right => Move2::Down,
            Move2::Down  => Move2::Left,
            Move2::Left  => Move2::Up,
        }
    }
    pub fn rotl(&self) -> Move2 {
        match self {
            Move2::Up    => Move2::Left,
            Move2::Left  => Move2::Down,
            Move2::Down  => Move2::Right,
            Move2::Right => Move2::Up,
        }
    }

}

// ShrunkPath stores the path string (UDLRLRLR etc.) but with each direction stored as only 2 bits
// It uses StackStack64, which has a limit to how long the path can be

#[derive(PartialOrd, Eq, Ord, Clone, Copy, PartialEq, Debug)]
pub struct ShrunkPath {
    count: u16,
    data: StackStack64,
}



impl ShrunkPath {
    pub fn new() -> Self {
        Self {
            count: 0,
            data: StackStack64::new(),
        }
    }
    pub fn clear(&mut self) {
        self.count = 0;
    }
    pub fn len(&self) -> u16 {
        self.count
    }
    pub fn from_path(path: &Vec::<Move2>) -> Self {
        let mut data = StackStack64::new();
        let mut x: u64 = 0;
        for i in 0..path.len() {
            if i % 32 == 0 && i != 0 {
                data.push(x);
                x=0;
            }
            x |= (path[i] as u64) << (2*(i%32));
        }
        if path.len() > 0 { data.push(x); }

        Self {
            count: path.len() as u16,
            data: data,
        }
    }
    pub fn push(&mut self, move1: &Move2) {
        if self.count%32==0 {
            // append new block
            self.data.push(*move1 as u64);
        } else {
            // modify existing block
            let idx = self.count as usize/32;
            let mut x = self.data.stack[idx];
            x |= (*move1 as u64) << (2*(self.count%32));
            self.data.stack[idx] = x;
        }
        self.count += 1;
    }
    pub fn push_u8(&mut self, move1: u8) {
        let move1 = move1 as u64;
        if self.count%32==0 {
            // append new block
            self.data.push(move1);
        } else {
            // modify existing block
            let idx = self.count as usize/32;
            let mut x = self.data.stack[idx];
            x |= (move1) << (2*(self.count%32));
            self.data.stack[idx] = x;
        }
        self.count += 1;
    }
    pub fn append_path(&mut self, path: &Vec::<Move2>) {
        for move1 in path {
            if self.count%32==0 {
                // append new block
                self.data.push(*move1 as u64);
            } else {
                // modify existing block
                let idx = self.count as usize/32;
                let mut x = self.data.stack[idx];
                x |= (*move1 as u64) << (2*(self.count%32));
                self.data.stack[idx] = x;
            }
            self.count += 1;
        }
    }
    // fn append_path_ss8(&mut self, ss: &StackStack8) {
    //     for i in 0..ss.next {
    //         self.push(&Move2::from_u8_unchecked(ss.stack[i]));
    //     }
    // }
    pub fn to_path(&self) -> Vec::<Move2> {
        let mut path = Vec::<Move2>::with_capacity(self.count as usize);
        for i in 0..self.count as usize {
            let block = self.data.stack[i/32];
            let shr = block >> (2*(i%32));
            path.push( Move2::from_u8_unchecked( shr as u8 & 0x03) );
        }

        path
    }
    pub fn score(&self) -> u64 {
        // actual movement scores 1 point
        // a rotation by 90 degrees scores 1000 points
        let mut score: u64 = self.len() as u64;
        let path = self.to_path();
        for idx in 1..self.len() as usize {
            let pair = [ path[idx-1], path[idx] ];
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
    pub fn calc_score(m0: &Move2, m: &Move2) -> u64 {
        // we are facing direction dir, do we need to rotate to get to m?
        let pair = [ m0, m ];
        if pair[0] == pair[1] { // only scores 1 for movement
            return 1;
        } else if *pair[0] == pair[1].rotr() || *pair[0] == pair[1].rotl() {
            return 1001;
        } else if *pair[0] == pair[1].rotr().rotr() {
            return 2001;  // terrible choice, making a u-turn!
        } else {
            println!("pair: {:?}", pair);
            panic!("shouldn't get here");
        }
    }
    pub fn to_string(&self) -> String {
        let path = self.to_path();
        let mut s: String = "".to_string();
        for m in path.iter() {
            s = s + &m.to_string();
        }
        return s;
    }
}


