// Box Bopper: Sokoban-like game
// Copyright David Atkinson 2020-2021
//
// level.rs: store level data and perform basic operations

use crate::path2::Move2;
use std::convert::TryInto;
use std::collections::BTreeSet;
use std::string::String;
use crate::vector::{Vector,VectorSm};
use crate::obj::Obj;

#[derive(Clone,PartialEq)]
pub struct LevelBitmap {
	pub w: u8,
	pub h: u8,
	pub bitmap: Vec<[u64;3]>,
}

impl LevelBitmap {
	pub fn new(w: usize, h: usize) -> LevelBitmap {
		Self {
			w: w.try_into().unwrap(),
			h: h.try_into().unwrap(),
			bitmap: vec![[0;3]; h],			// level is up to 141 chars wide
		}
	}
	pub fn set_v(&mut self, v: Vector) {
		let y = v.1 as usize;
		let xn = v.0 as usize / 64;
		let xr = v.0 as usize % 64;
		self.bitmap[y][xn] |= 1 << xr;
	}
	pub fn clear_v(&mut self, v: Vector) {
		let y = v.1 as usize;
		let xn = v.0 as usize/ 64;
		let xr = v.0 as usize % 64;
		self.bitmap[y][xn] &= !(1 << xr);
	}
	pub fn get_v(&self, v: Vector) -> bool {
		let y = v.1 as usize;
		let xn = v.0 as usize / 64;
		let xr = v.0 as usize % 64;
		self.bitmap[y][xn] & (1 << xr) != 0
	}
	pub fn to_string(&self) -> String {
		let mut s = String::new();
		for y in 0..self.h {
			for x in 0..self.w {
				if self.get_v(Vector(x.into(),y.into())) {
					s.push('#');
				} else {
					s.push('.');
				}
			}
			s += "\n";
		}
		s
	}
}


#[derive(Clone,PartialEq)] //,PartialOrd
pub struct Level {
	pub w: u16,
	pub h: u16,
	pub deer_pos: Vector,
	pub deer_dir: Move2,
	pub end_pos: Vector,
	pub start_pos: Vector,
	data: Vec::<Obj>,
	wall_pts: BTreeSet::<Vector>,
	pub(crate) wall_bmp: LevelBitmap,
}



impl Level {
	pub fn get_obj_at_pt(&self, pt: &Vector) -> Obj {
		if self.wall_bmp.get_v(*pt) {
			return Obj::Wall;
		}
		if self.deer_pos == *pt {
			return Obj::Deer;
		}
		// should be just a space
		return Obj::Space;
		//self.data[(pt.0 as usize) + (pt.1 as usize) * (self.w as usize)]
	}
	pub fn set_obj_at_pt(&mut self, pt: &Vector, obj: Obj) {
		self.data[(pt.0 as usize) + (pt.1 as usize) * (self.w as usize)] = obj;
	}
	pub fn get_obj_at_pt_checked(&self, pt: &Vector) -> Obj {
		if pt.0 < 0 || pt.0 >= self.w as i32 || pt.1 < 0 || pt.1 >= self.h as i32 {
			Obj::Wall
		} else {
			self.get_obj_at_pt(pt)
		}
	}
	pub fn set_obj_at_pt_checked(&mut self, pt: &Vector, obj: Obj) {
		if pt.0 < 0 || pt.0 >= self.w as i32 || pt.1 < 0 || pt.1 >= self.h as i32 {
			panic!("set_obj_at_pt_checked(): out of bounds pt");
		} else {
			self.set_obj_at_pt(pt, obj);
		}
	}
	pub fn have_win_condition(&self) -> bool {
		// deer should be in finish position
		self.deer_pos == self.end_pos
	}
	pub fn from_str(level_str: &str) -> Result<Level, &str> {
		let mut h: u16 = 0;
		let mut w: u16 = 0;
		let mut data = Vec::<Obj>::with_capacity(128);
		let mut start_pos: Option<Vector> = None;
		let mut end_pos: Option<Vector> = None;
	
		for (count, line) in level_str.lines().enumerate() {
			let txt = line;
			if count == 0 {
				// read in length
				w = txt.len() as u16;			
			}
			// check length equal to w
			if txt.len() == w as usize {
				// split line into characters
				for (i,c) in txt.char_indices() {		// chars() is iterator
					if c == 'S' {
						// found start deer_pos
						if start_pos.is_none() {
							start_pos = Some(Vector(i.try_into().unwrap(),h.try_into().unwrap()));
						} else {
							return Err("More than one start found!");
						}
						data.push(Obj::Space);
					} else if c == 'E' {
						// found end_pos
						if end_pos.is_none() {
							end_pos = Some(Vector(i.try_into().unwrap(),h.try_into().unwrap()));
						} else {
							return Err("More than one end found!");
						}
						data.push(Obj::Space);
					} else {
						data.push(Obj::from_char(&c));
					}
				}
				h += 1;
			} else {
				panic!("unexpected line width");
			}
		}

		// remove the borders
		// let mut tdata = Vec::<Obj>::new();
		// for y in 1..(h-1) as usize {
		// 	for x in 1..(w-1) as usize {
		// 		tdata.push(data[y*w as usize+x]);
		// 	}
		// }
		// let data = tdata;

		// w -= 2;
		// h -= 2;
		if start_pos.is_none() || end_pos.is_none() {
			return Err("Start and/or end not found in level!");
		}
		let	end_pos = end_pos.unwrap();
		let start_pos = start_pos.unwrap();

		if w < 3 || h < 3 {
			//println!("Dimensions: {} x {}", w, h);
			return Err("Width and Height must be at least 3!");
		}
		// if w > 127 || h > 127 || w * h > 256 {
		// 	//println!("Dimensions: {} x {}", w, h);
		// 	return Err("Level too big! Maximum width 127. Maximum height 127. Maximum width * height 256.");
		// }

		let mut level = Level {
			w: w,
			h: h,
			deer_pos: start_pos,
			deer_dir: Move2::Right,
			start_pos: start_pos,
			end_pos: end_pos,
			wall_pts: BTreeSet::new(),
			wall_bmp: LevelBitmap::new(w.into(),h.into()),
			data: data,
		};
		Self::init_level(&mut level);
		return Ok(level);
	}
	pub fn init_level(&mut self) {
		// set up wall_pts and wall_bitmap
		for y in 0..self.h {
			for x in 0..self.w {
				if self.get_obj_at_pt(&Vector(x.into(),y.into())) == Obj::Wall {
					self.wall_pts.insert(Vector(x.into(),y.into()));
					self.wall_bmp.set_v(Vector(x.into(), y.into()));
				}
			}
		}
	}
	pub fn clear_deer(&mut self) {
		// clear the deer from the level to make certain things easier
		let pt = self.deer_pos;
		let obj = self.get_obj_at_pt(&pt);
		let obj2 = match obj {
			Obj::Deer => Obj::Space,
			_ => obj,
		};
		self.set_obj_at_pt(&pt, obj2);
	}
	pub fn place_deer(&mut self) {
		// place the deer in the level data
		let pt = self.deer_pos;
		let obj = self.get_obj_at_pt(&self.deer_pos);
		let obj2 = match obj {
			Obj::Space => Obj::Deer,
			_ => panic!("Deer cannot be there!"),
		};
		self.set_obj_at_pt(&pt, obj2);
	}
	pub fn in_wall_pts(&self, v: &Vector) -> bool {
		self.wall_pts.contains(v)
	}
	pub fn in_wall_pts8(&self, v: &VectorSm) -> bool {
		match self.wall_pts.get(&v.intov()) {
			Some(_) => true,
			_ => false,
		}
	}
	pub fn eq_data(&self, b: &Level) -> bool {
		self.data == b.data && self.deer_pos == b.deer_pos
	}
	pub fn vector_in_bounds(&self, v: &Vector) -> bool {
		( v.0 | v.1 | (self.w as i32 - v.0 - 1) | (self.h as i32 - v.1 - 1)  ) >= 0
	}
	pub fn vector_in_bounds8(&self, v: &VectorSm) -> bool {
		v.0 >= 0 && v.0 < (self.w as i8) && v.1 >= 0 && v.1 < (self.h as i8)
	}
	pub fn to_string(&self) -> String {
		let mut s = String::new();
		for y in 0..self.h as i32 {
			for x in 0..self.w as i32 {
				s += &self.get_obj_at_pt(&Vector(x,y)).to_char().to_string();
			}
			s += "\n";
		}
		s
	}
	pub fn has_space_at(&self, v: Vector) -> bool {
		!(!self.vector_in_bounds(&v) || self.wall_bmp.get_v(v))
	}
	pub fn get_path_pts(&self, path: &Vec<Move2>) -> Vec<Vector> {
		let mut pos = self.start_pos.clone();
		let mut vecs = Vec::<Vector>::new();
		vecs.push(pos);
		for m in path {
			pos = pos.apply_dir(m);
			vecs.push(pos);
		}
		vecs
	}
}

