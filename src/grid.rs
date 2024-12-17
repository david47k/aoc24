use crate::vector::Vector;
use itertools::Itertools;

pub struct Grid {
    pub w: i32,
    pub h: i32,
    pub data: Vec<u8>,
}

pub const NDIR_U:  usize = 0;
pub const NDIR_UR: usize = 1;
pub const NDIR_R:  usize = 2;
pub const NDIR_DR: usize = 3;
pub const NDIR_D:  usize = 4;
pub const NDIR_DL: usize = 5;
pub const NDIR_L:  usize = 6;
pub const NDIR_UL: usize = 7;

pub const NDIRS: [Vector; 8] = [ Vector(0,-1), Vector(1,-1), Vector(1,0), Vector(1,1),
Vector(0,1), Vector(-1,1), Vector(-1,0), Vector(-1,-1) ];

impl Grid {
    pub fn new(w: i32, h: i32) -> Self {
        Self {
            w,
            h,
            data: vec![b'.'; w as usize * h as usize],
        }
    }
    pub fn new_with(w: i32, h: i32, c: u8) -> Self {
        Self {
            w,
            h,
            data: vec![c as u8; w as usize * h as usize],
        }
    }
    pub fn from_str(s: &str) -> Self {
        let rows = s.lines().collect::<Vec<&str>>();
        let data: Vec<u8> = rows.iter().map(|r| r.bytes().collect::<Vec<u8>>()).flatten().collect();
        Self {
            h: rows.len() as i32,
            w: rows[0].len() as i32,
            data,
        }
    }
    pub fn has_xy(&self, xy: &Vector) -> bool {
        xy.0 >= 0 && xy.0 < self.w && xy.1 >= 0 && xy.1 < self.h
    }
    pub fn get(&self, xy: &Vector) -> Option<u8> {
        if xy.is_valid(&self) {
            return Some(self.data[xy.1 as usize * self.w as usize + xy.0 as usize]);
        }
        None
    }
    pub fn get_unchecked(&self, xy: &Vector) -> u8 {
        self.data[xy.1 as usize * self.w as usize + xy.0 as usize]
    }
    pub fn get_neighbours(&self, xy: &Vector) -> Vec<Option<u8>> {
        // U, UR, R, DR, D, DL, L, UL.
        NDIRS.iter().map(|d| self.get(&xy.add(&d))).collect_vec()
    }
    pub fn put(&mut self, xy: &Vector, value: u8) -> bool {
        if xy.is_valid(&self) {
            self.data[xy.1 as usize * self.w as usize + xy.0 as usize] = value;
            return true;
        }
        false
    }
    pub fn replace_fn(&mut self, f: fn(u8) -> u8 ) {
        for y in 0..self.h {
            for x in 0..self.w {
                let pt = &mut self.data[y as usize * self.w as usize + x as usize];
                *pt = f(*pt);
            }
        }
    }
    pub fn put_unchecked(&mut self, xy: &Vector, value: u8) {
        self.data[xy.1 as usize * self.w as usize + xy.0 as usize] = value;
    }
    pub fn put_unchecked_t(&mut self, xy: (isize,isize), value: u8) {
        self.data[xy.1 as usize * self.w as usize + xy.0 as usize] = value;
    }
    pub fn find(&self, value: u8) -> Vec<Vector> {
        let mut results: Vec<Vector> = Vec::new();
        for y in 0..self.h {
            for x in 0..self.w {
                if self.data[y as usize * self.w as usize + x as usize] == value {
                    results.push(Vector::new(x,y));
                }
            }
        }
        results
    }
    pub fn find_fn(&self, f: fn(u8) -> bool) -> Vec<Vector> {
        let mut results: Vec<Vector> = Vec::new();
        for y in 0..self.h {
            for x in 0..self.w {
                if f(self.data[y as usize * self.w as usize + x as usize]) {
                    results.push(Vector::new(x,y));
                }
            }
        }
        results
    }
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for y in 0..self.h {
            s += &String::from_utf8(self.data[y as usize * self.w as usize..(y as usize + 1) * self.w as usize].to_vec()).expect("valid string");
            s += "\n";
        }
        s
    }
    pub fn to_string_with_pt(&self, pt: &Vector) -> String {
        let mut s = String::new();
        let mut d = self.data.clone();
        d[pt.1 as usize * self.w as usize + pt.0 as usize] = b'@';
        for y in 0..self.h {
            let rs = String::from_utf8(d[y as usize * self.w as usize..(y as usize + 1) * self.w as usize].to_vec()).expect("valid string");
            s += &rs;
            s += "\n";
        }
        s
    }
}


