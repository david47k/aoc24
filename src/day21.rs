use itertools::Itertools;
use crate::vector::{*};

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
	pub fn button_press(&mut self, c: char, mut method: Method) -> Vec<char> {
		let mut path: Vec<char> = vec![];
		let dest = self.c_to_v(c);
		let diff = dest.sub(&self.posn);
		let nogo = self.no_go();
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
		path
	}
	pub fn do_path(&mut self, path: &Vec<char>) -> Vec<char> {
		let save = self.posn.clone();
		let mut shortest_path: Vec<char> = vec![];
		let mut shortest_path_len: usize = 100_000;
		let mut shortest_end_posn = self.posn.clone();
		for combo in [Method::A, Method::V].into_iter().combinations_with_replacement(path.len()) {
			self.posn = save;
			let mut new_path: Vec<char> = vec![];
			for i in 0..path.len() {
				new_path.extend( self.button_press(path[i], combo[i]) );
			}
			if new_path.len() < shortest_path_len {
				shortest_path = new_path;
				shortest_path_len = shortest_path.len();
				shortest_end_posn = self.posn.clone();
			}
		}
		self.posn = shortest_end_posn;
		shortest_path
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

fn path_to_string(path: &Vec<char>) -> String {
	path.into_iter().collect::<String>()
}

pub fn day21(input: &String) -> (String, String) {
	let codes_s: Vec<&str> = input.lines().collect_vec();
	let codes: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
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
	(p1soln.to_string(), "no part 2 solution".to_string())
}
