#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub struct XY {
    pub x: isize,
    pub y: isize,
}
impl XY {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub fn sub(&self, b: &XY) -> Self {
        Self {
            x: self.x - b.x,
            y: self.y - b.y,
        }
    }
    pub fn add(&self, b: &XY) -> Self {
        Self {
            x: self.x + b.x,
            y: self.y + b.y,
        }
    }
    pub fn neg(&self) -> XY {
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
    pub fn from_str(s: &str) -> Self {
        let rows = s.lines().collect::<Vec<&str>>();
        let data: Vec<Vec<u8>> = rows.iter().map(|r| r.bytes().collect::<Vec<u8>>()).collect();
        Self {
            h: rows.len() as isize,
            w: rows[0].len() as isize,
            data,
        }
    }
    pub fn has_xy(&self, xy: &XY) -> bool {
        xy.x >= 0 && xy.x < self.w && xy.y >= 0 && xy.y < self.h
    }
    pub fn get(&self, xy: &XY) -> Option<u8> {
        if xy.is_valid(&self) {
            return Some(self.data[xy.y as usize][xy.x as usize]);
        }
        None
    }
    pub fn get_unchecked(&self, xy: &XY) -> u8 {
        self.data[xy.y as usize][xy.x as usize]
    }
    pub fn put(&mut self, xy: &XY, value: u8) -> bool {
        if xy.is_valid(&self) {
            self.data[xy.y as usize][xy.x as usize] = value;
            return true;
        }
        false
    }
    pub fn find(&self, value: u8) -> Vec<XY> {
        let mut results: Vec<XY> = Vec::new();
        for y in 0..self.h {
            for x in 0..self.w {
                if self.data[y as usize][x as usize] == value {
                    results.push(XY::new(x,y));
                }
            }
        }
        results
    }
    pub fn find_fn(&self, f: fn(u8) -> bool) -> Vec<XY> {
        let mut results: Vec<XY> = Vec::new();
        for y in 0..self.h {
            for x in 0..self.w {
                if f(self.data[y as usize][x as usize]) {
                    results.push(XY::new(x,y));
                }
            }
        }
        results
    }
}


