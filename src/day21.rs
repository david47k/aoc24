use itertools::Itertools;
use crate::vector::{*};
use std::collections::BTreeMap;

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

const MINIVEC_SIZE: usize = 5;
#[derive(Copy,Clone,Ord,PartialOrd,Eq,PartialEq,Debug)]
struct MiniVec<T> {
	data: [T; MINIVEC_SIZE],
	len: usize,
}

impl<T: Copy> MiniVec<T> {
	fn new(z: T) -> Self {
		Self {
			data: [ z; MINIVEC_SIZE ], // unsafe { std::mem::MaybeUninit::uninit().assume_init() },
			len: 0,
		}
	}
	fn push(&mut self, x: T) {
		if self.len == MINIVEC_SIZE {
			panic!("exceeded minivec size");
		}
		self.data[self.len] = x;
		self.len += 1;
	}
	fn extend(&mut self, x: T, n: usize) {
		for _ in 0..n {
			self.push(x);
		}
	}
	fn to_vec(&self) -> Vec<T> {
		let mut r: Vec<T> = vec![];
		for i in 0..self.len {
			r.push(self.data[i]);
		}
		r
	}
}

#[derive(Clone)]
struct RobotChain {
	pub robots: Vec<Robot>,
	pub pcache: BTreeMap<(Robot,MiniVec<char>), usize>,
}

impl RobotChain {
	fn do_path(&mut self, c: char, depth: usize) -> usize {
		let path: MiniVec<char>;

		// do button_press
		path = self.robots[depth].button_press_2(c);

		let new_depth = depth + 1;

		// check if we are at max depth
		if new_depth == self.robots.len() {
			return path.len;
		}

		// bubble up each character one at a time, to max depth
		// sum the move count and bubble down
		let mut count: usize = 0;

		if self.robots.len() < 12 || depth < self.robots.len() - 12 {
			for i in 0..path.len {
				count += self.do_path(path.data[i], new_depth);
			}
		} else {
			count = self.do_path_wide_cached(&path, new_depth);
		}
										// return the total
		count
	}
	fn do_path_wide_cached(&mut self, opath: &MiniVec<char>, depth: usize) -> usize {
		let rdata = self.robots[depth];
		if let Some(r) = self.pcache.get(&(rdata.clone(),opath.clone())) {
			return r.to_owned();
		}

		let mut path = opath.to_vec();

		// do button_press
		for d in depth..self.robots.len() {
			let mut npath: Vec<char> = vec![];
			for c in path.into_iter() {
				self.robots[d].button_press_3(c, &mut npath);
			}
			path = npath;
		}

		// return cached path
		self.pcache.insert((rdata,opath.clone()), path.len());
		path.len()
	}
}

#[derive(Clone,Copy,Ord,PartialOrd,Eq,PartialEq)]
struct Robot {
	pub posn: Vector,
	pub control_type: ControlType,
}

#[repr(u8)]
#[derive(Clone,Copy,PartialEq,Eq,Debug)]
enum Method {
	A,
	V,
}

#[repr(u8)]
#[derive(Clone,Copy,PartialEq,Eq,Debug,Ord,PartialOrd)]
enum ControlType {
	Numpad,
	Directional,
}

impl Robot {
	pub fn no_go(&self) -> Vector {
		if self.control_type == ControlType::Numpad {
			Vector(0,3)
		} else {
			Vector(0,0)
		}
	}
	pub fn c_to_v_numpad(c: char) -> Vector {
		match c {
			'7' => Vector(0,0),
			'8' => Vector(1,0),
			'9' => Vector(2,0),
			'4' => Vector(0,1),
			'5' => Vector(1,1),
			'6' => Vector(2,1),
			'1' => Vector(0,2),
			'2' => Vector(1,2),
			'3' => Vector(2,2),
			'0' => Vector(1,3),
			'A' => Vector(2,3),
			_ => panic!(),
		}
	}
	pub fn c_to_v_directional(c: char) -> Vector {
		match c {
			'^' => Vector(1,0),
			'A' => Vector(2,0),
			'<' => Vector(0,1),
			'v' => Vector(1,1),
			'>' => Vector(2,1),
			_ => panic!(),
		}
	}
	pub fn c_to_v(&self, c: char) -> Vector {
		if self.control_type == ControlType::Numpad {
			Self::c_to_v_numpad(c)
		} else {
			Self::c_to_v_directional(c)
		}
	}
	pub fn button_press_2(&mut self, c: char) -> MiniVec<char> {
		// best path is < , then ^/v, then >
		// we condense this into simply method Vertical first or Across first
		let mut path: MiniVec<char> = MiniVec::new('.');
		let dest = self.c_to_v(c);
		let diff = dest.sub(&self.posn);
		let nogo = self.no_go();
		let mut method: Method;
		if diff.0 < 0 {
			method = Method::A;
		} else {
			method = Method::V;
		}
		// avoid passing the no-go zone
		if self.posn.0 == nogo.0 && dest.1 == nogo.1 {
			method = Method::A;
		}
		if self.posn.1 == nogo.1 && dest.0 == nogo.0 {
			method = Method::V;
		}
		if method == Method::V { 	// go vertical first
			if diff.1 < 0 {
				path.extend('^', diff.1.abs() as usize);
			} else {
				path.extend('v', diff.1.abs() as usize);
			}
			if diff.0 < 0 {
				path.extend('<', diff.0.abs() as usize );
			} else {
				path.extend('>', diff.0.abs() as usize );
			}
		} else { 		// go horizontal first
			if diff.0 < 0 {
				path.extend('<', diff.0.abs() as usize );
			} else {
				path.extend('>', diff.0.abs() as usize );
			}
			if diff.1 < 0 {
				path.extend('^', diff.1.abs() as usize)
			} else {
				path.extend('v', diff.1.abs() as usize)
			}
		}
		self.posn = dest;
		path.push('A');
		path
	}
	pub fn button_press_3(&mut self, c: char, path: &mut Vec::<char>) {
		// best path is < , then ^/v, then >
		// we condense this into simply method Vertical first or Across first
		let dest = self.c_to_v(c);
		let diff = dest.sub(&self.posn);
		let nogo = self.no_go();
		let mut method: Method;
		if diff.0 < 0 {
			method = Method::A;
		} else {
			method = Method::V;
		}
		// avoid passing the no-go zone
		if self.posn.0 == nogo.0 && dest.1 == nogo.1 {
			method = Method::A;
		}
		if self.posn.1 == nogo.1 && dest.0 == nogo.0 {
			method = Method::V;
		}
		if method == Method::V { 	// go vertical first
			if diff.1 < 0 {
				path.extend(vec!['^'; diff.1.abs() as usize]);
			} else {
				path.extend(vec!['v'; diff.1.abs() as usize]);
			}
			if diff.0 < 0 {
				path.extend(vec!['<'; diff.0.abs() as usize] );
			} else {
				path.extend(vec!['>'; diff.0.abs() as usize] );
			}
		} else { 		// go horizontal first
			if diff.0 < 0 {
				path.extend(vec!['<'; diff.0.abs() as usize] );
			} else {
				path.extend(vec!['>'; diff.0.abs() as usize] );
			}
			if diff.1 < 0 {
				path.extend(vec!['^'; diff.1.abs() as usize])
			} else {
				path.extend(vec!['v'; diff.1.abs() as usize])
			}
		}
		self.posn = dest;
		path.push('A');
	}
	pub fn new(control_type: ControlType) -> Self {
		let v = if control_type == ControlType::Numpad {
			Vector(2,3)
		} else {
			Vector(2,0)
		};
		Self { posn: v, control_type }
	}
}

pub fn day21(input: &String) -> (String, String) {
	let codes_s: Vec<&str> = input.lines().collect_vec();
	let codes: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
	println!("part 1 calculating...");
	let robot1 = Robot::new(ControlType::Numpad);
	let robot2 = Robot::new(ControlType::Directional);
	let robot3 = Robot::new(ControlType::Directional);
	let mut robot_chain = RobotChain {
		robots: vec![robot1, robot2, robot3],
		pcache: BTreeMap::new(),
	};
	let mut p1soln: usize = 0;
	for (i,&ref code) in codes.iter().enumerate() {
		println!("'{}'...",  codes_s[i]);
		let mut count = 0;
		for c in code.iter() {
			count += robot_chain.do_path(*c, 0);
		}
		let n1 = codes_s[i][0..3].parse::<usize>().unwrap();
		let complexity: usize = count * (codes_s[i][0..3]).parse::<usize>().unwrap();
		println!("complexity = {} * length {} = {}", n1, count, complexity);
		p1soln += complexity;
	}

	println!("part 1 solution: {}", p1soln);

	let t0 = crate::time::get_time_ms();

	println!("part 2 calculating...");

	// build a robot chain

	let mut robot_vec = vec![ Robot::new(ControlType::Numpad) ];
	robot_vec.extend(vec![ Robot::new(ControlType::Directional); 25]);
	let mut robot_chain = RobotChain {
		robots: robot_vec,
		pcache: BTreeMap::new(),
	};

	let mut p2soln: usize = 0;
	for (i,&ref code) in codes.iter().enumerate() {
		let mut count = 0;
		println!("'{}'...", codes_s[i]);
		for c in code.iter() {
			count += robot_chain.do_path(*c, 0);
		}
		let n1 = codes_s[i][0..3].parse::<usize>().unwrap();
		let complexity: usize = count * (codes_s[i][0..3]).parse::<usize>().unwrap();
		println!("complexity = {} * length {} = {}", n1, count, complexity);
		p2soln += complexity;
	}

	let t1 = crate::time::get_time_ms();
	println!("part 2 solution: {}", p2soln);
	println!("time for part 2 : {:.0} ms", t1 - t0);

	(p1soln.to_string(), p2soln.to_string())
}
