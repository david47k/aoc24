use crate::vector::Vector;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub struct VectorOld {
    pub x: isize,
    pub y: isize,
}
impl VectorOld {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub fn sub(&self, b: &VectorOld) -> Self {
        Self {
            x: self.x - b.x,
            y: self.y - b.y,
        }
    }
    pub fn add(&self, b: &VectorOld) -> Self {
        Self {
            x: self.x + b.x,
            y: self.y + b.y,
        }
    }
    pub fn neg(&self) -> VectorOld {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
    pub fn is_valid(&self, grid: &Grid) -> bool {
        self.x >= 0 && self.x < grid.w && self.y >= 0 && self.y < grid.h
    }
    pub fn to_string(&self) -> String {
        format!("({},{})", self.x, self.y)
    }
}

pub struct Grid {
    pub w: isize,
    pub h: isize,
    data: Vec<Vec<u8>>,
}

impl Grid {
    pub fn new(w: isize, h: isize) -> Self {
        Self {
            w,
            h,
            data: vec![vec![0; w as usize]; h as usize],
        }
    }
    pub fn new_with(w: isize, h: isize, c: u8) -> Self {
        Self {
            w,
            h,
            data: vec![vec![c; w as usize]; h as usize],
        }
    }
    pub fn from_str(s: &str) -> Self {
        let rows = s.lines().collect::<Vec<&str>>();
        let data: Vec<Vec<u8>> = rows.iter().map(|r| r.bytes().collect::<Vec<u8>>()).collect();
        Self {
            h: rows.len() as isize,
            w: rows[0].len() as isize,
            data,
        }
    }
    pub fn has_xy(&self, xy: &Vector) -> bool {
        xy.0 >= 0 && xy.0 < self.w && xy.1 >= 0 && xy.1 < self.h
    }
    pub fn get(&self, xy: &Vector) -> Option<u8> {
        if xy.is_valid(&self) {
            return Some(self.data[xy.1 as usize][xy.0 as usize]);
        }
        None
    }
    pub fn get_unchecked(&self, xy: &Vector) -> u8 {
        self.data[xy.1 as usize][xy.0 as usize]
    }
    pub fn put(&mut self, xy: &Vector, value: u8) -> bool {
        if xy.is_valid(&self) {
            self.data[xy.1 as usize][xy.0 as usize] = value;
            return true;
        }
        false
    }
    pub fn replace_fn(&mut self, f: fn(u8) -> u8 ) {
        for y in 0..self.h {
            for x in 0..self.w {
                let pt = &mut self.data[y as usize][x as usize];
                *pt = f(*pt);
            }
        }
    }
    pub fn put_unchecked(&mut self, xy: &Vector, value: u8) {
        self.data[xy.1 as usize][xy.0 as usize] = value;
    }
    pub fn put_unchecked_t(&mut self, xy: (isize,isize), value: u8) {
        self.data[xy.1 as usize][xy.0 as usize] = value;
    }
    pub fn find(&self, value: u8) -> Vec<Vector> {
        let mut results: Vec<Vector> = Vec::new();
        for y in 0..self.h {
            for x in 0..self.w {
                if self.data[y as usize][x as usize] == value {
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
                if f(self.data[y as usize][x as usize]) {
                    results.push(Vector::new(x,y));
                }
            }
        }
        results
    }
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for y in 0..self.h {
            s += &String::from_utf8(self.data[y as usize].clone()).expect("valid string");
            s += "\n";
        }
        s
    }
}


