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

struct Cache {
	pub cache: BTreeMap<(Vector, u8), (Vector,Vec<u8>)>,
	pub pcache: BTreeMap<(Vector, Vec<u8>), (Vector,Vec<u8>)>,
}

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

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
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
	pub fn c_to_v_numpad(c: u8) -> Vector {
		match c {
			b'7' => Vector(0,0),
			b'8' => Vector(1,0),
			b'9' => Vector(2,0),
			b'4' => Vector(0,1),
			b'5' => Vector(1,1),
			b'6' => Vector(2,1),
			b'1' => Vector(0,2),
			b'2' => Vector(1,2),
			b'3' => Vector(2,2),
			b'0' => Vector(1,3),
			b'A' => Vector(2,3),
			_ => panic!(),
		}
	}
	pub fn c_to_v_directional(c: u8) -> Vector {
		match c {
			b'^' => Vector(1,0),
			b'A' => Vector(2,0),
			b'<' => Vector(0,1),
			b'v' => Vector(1,1),
			b'>' => Vector(2,1),
			_ => panic!(),
		}
	}
	pub fn c_to_v(&self, c: u8) -> Vector {
		if self.control_type == ControlType::Numpad {
			Self::c_to_v_numpad(c)
		} else {
			Self::c_to_v_directional(c)
		}
	}
	pub fn button_press(&mut self, c: u8) -> Vec<u8> {
		// priority order: L, U, D, R.
		let mut path: Vec<u8> = vec![];
		let dest = self.c_to_v(c);
		let diff = dest.sub(&self.posn);
		let nogo = self.no_go();
		let mut method: Method;
		if diff.0 < 0 {
			method = Method::A;	// across first
		} else {
			method = Method::V; // vertical first
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
				path.extend(vec![b'^'; diff.1.abs() as usize]);
			} else {
				path.extend(vec![b'v'; diff.1.abs() as usize]);
			}
			if diff.0 < 0 {
				path.extend(vec![b'<'; diff.0.abs() as usize] );
			} else {
				path.extend(vec![b'>'; diff.0.abs() as usize] );
			}
		} else { 		// go horizontal first
			if diff.0 < 0 {
				path.extend(vec![b'<'; diff.0.abs() as usize] );
			} else {
				path.extend(vec![b'>'; diff.0.abs() as usize] );
			}
			if diff.1 < 0 {
				path.extend(vec![b'^'; diff.1.abs() as usize])
			} else {
				path.extend(vec![b'v'; diff.1.abs() as usize])
			}
		}
		self.posn = dest;
		path.push(b'A');
		path
	}
	pub fn button_press_2(&mut self, c: u8, cache: &mut Cache) -> Vec<u8> {
		if self.control_type == ControlType::Directional {
			let lookup = cache.cache.get(&(self.posn, c));
			if let Some(r) = lookup {
				self.posn = r.0;
				return r.1.clone();
			}
		}

		// priority order: L, U, D, R.
		let mut path: Vec<u8> = vec![];
		let dest = self.c_to_v(c);
		let diff = dest.sub(&self.posn);
		let nogo = self.no_go();
		let mut method: Method;
		if diff.0 < 0 {
			method = Method::A;	// across first
		} else {
			method = Method::V; // vertical first
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
				path.extend(vec![b'^'; diff.1.abs() as usize]);
			} else {
				path.extend(vec![b'v'; diff.1.abs() as usize]);
			}
			if diff.0 < 0 {
				path.extend(vec![b'<'; diff.0.abs() as usize] );
			} else {
				path.extend(vec![b'>'; diff.0.abs() as usize] );
			}
		} else { 		// go horizontal first
			if diff.0 < 0 {
				path.extend(vec![b'<'; diff.0.abs() as usize] );
			} else {
				path.extend(vec![b'>'; diff.0.abs() as usize] );
			}
			if diff.1 < 0 {
				path.extend(vec![b'^'; diff.1.abs() as usize])
			} else {
				path.extend(vec![b'v'; diff.1.abs() as usize])
			}
		}
		path.push(b'A');
		if self.control_type == ControlType::Directional {
			cache.cache.insert((self.posn, c), (dest, path.clone()));
		}
		self.posn = dest;
		path
	}
	pub fn do_path_c(&mut self, path: &Vec<u8>, cache: &mut Cache) -> Vec<u8> {
		let mut new_path: Vec<u8> = Vec::with_capacity(path.len()*5);
		for chunk in path.chunks(40) {
			let v = Vec::from(chunk);
			if self.control_type == ControlType::Directional {
				if let Some(r) = cache.pcache.get(&(self.posn, v.clone())) {
					self.posn = r.0;
					new_path.extend(r.1.clone());
					continue;
				}
			}
			let mut tpath: Vec<u8> = vec![];
			for i in 0..v.len() {
				tpath.extend(self.button_press_2(v[i], cache));
			}
			if self.control_type == ControlType::Directional {
				cache.pcache.insert((self.posn, v.clone()), (self.posn, tpath.clone()));
			}
			new_path.extend(tpath);
		}
		new_path
	}
	pub fn do_path(&mut self, path: &Vec<u8>) -> Vec<u8> {
		let mut tpath: Vec<u8> = vec![];
		for i in 0..path.len() {
			tpath.extend(self.button_press(path[i]));
		}
		tpath
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

fn path_to_string(path: &Vec<u8>) -> String {
	path.into_iter().map(|&u| char::from(u)).collect::<String>()
}

pub fn day21(input: &String) -> (String, String) {
	let codes_s: Vec<&str> = input.lines().collect_vec();
	let codes: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();
	println!("part 1");
	let mut robot1 = Robot::new(ControlType::Numpad);
	let mut robot2 = Robot::new(ControlType::Directional);
	let mut robot3 = Robot::new(ControlType::Directional);
	let mut p1soln: usize = 0;
	for (i,&ref code) in codes.iter().enumerate() {
		let p1 = robot1.do_path(code);
		//println!("path1: {}", path_to_string(&p1));
		let p2 = robot2.do_path(&p1);
		//println!("path2: {}", path_to_string(&p2));
		let path = robot3.do_path(&p2);
		println!("path for {} of length {}: {}", codes_s[i], path.len(), path_to_string(&path));
		let n1 = codes_s[i][0..3].parse::<usize>().unwrap();
		let complexity: usize = path.len() * (codes_s[i][0..3]).parse::<usize>().unwrap();
		println!("complexity {} * {}: {}", n1, path.len(), complexity);
		p1soln += complexity;
	}

	println!("part 1 solution: {}", p1soln);

	println!("part 2");

	let mut cache = Cache {
		cache: BTreeMap::new(),
		pcache: BTreeMap::new(),
	};

	let mut robots: Vec<Robot> = vec![];
	robots.push( Robot::new(ControlType::Numpad) );
	for _i in 0..25 {
		robots.push( Robot::new(ControlType::Directional) );
	}

	let mut p2soln: usize = 0;
	for (i,&ref codev) in codes.iter().enumerate() {
		println!("code: {}", codes_s[i]);
		let code = codev.clone();
		let mut plen = 0;
		for c in code.iter() {
			println!("{}: ", char::from(*c));
			let mut p = vec![*c];
			let mut lens = vec![];
			for (i,r) in robots.iter_mut().enumerate() {
				println!("{}...", i);
				p = r.do_path_c(&p, &mut cache);
				lens.push(p.len());
				println!("  len: {}", p.len());
				if i > 0 {
					println!("  diff: {}", p.len() - lens[i-1]);
					println!("  ratio: {}", p.len() as f64 / lens[i-1] as f64);
				}
			}
			println!("total len for this input: {}", p.len());
			plen += p.len();
		}
		println!("path for {} of length {}", codes_s[i], plen);
		let n1 = codes_s[i][0..3].parse::<usize>().unwrap();
		let complexity: usize = plen * (codes_s[i][0..3]).parse::<usize>().unwrap();
		println!("complexity {} * {}: {}", n1, plen, complexity);
		p2soln += complexity;
	}

	println!("part 2 solution: {}", p2soln);

	(p1soln.to_string(), p2soln.to_string())
}
