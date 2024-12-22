use itertools::Itertools;
use crate::path2::ShrunkPath;
//use itertools::Itertools;
//use std::collections::{*};
//use crate::grid::{*};
use crate::vector::{*};

// Robot1 is closest one to the numpad
struct Robot1 {
	pub posn: Vector,
}

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+

impl Robot1 {
	pub fn c_to_v(c: char) -> Vector {
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
	pub fn move_and_get_path(&mut self, c: char) -> Vec<char> {
		let mut path: Vec<char> = vec![];
		let dest = Self::c_to_v(c);
		let diff = dest.sub(&self.posn);
		self.posn = dest;
		if diff.1 < 0 { 		// if we are moving up, go up first then left/right
			path.extend(vec!['^'; diff.1.abs() as usize]);
			if diff.0 < 0 {
				path.extend(vec!['<'; diff.0.abs() as usize] );
			} else {
				path.extend(vec!['>'; diff.0 as usize] );
			}
		} else { 		// if we are moving down, go left/right first then down
			if diff.0 < 0 {
				path.extend(vec!['<'; diff.0.abs() as usize] );
			} else {
				path.extend(vec!['>'; diff.0 as usize] );
			}
			path.extend(vec!['v'; diff.1 as usize])
		}
		path.push('A');
		path
	}
	pub fn new() -> Self {
		Self { posn: Vector(2,3) }
	}
}

// RobotM is the interMediate robots / controller
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
struct RobotM {
	pub posn: Vector,
}

impl RobotM {
	pub fn new() -> Self {
		Self { posn: Vector(2,0), }
	}
	pub fn c_to_v(c: char) -> Vector {
		match c {
			'^' => Vector(1,0),
			'A' => Vector(2,0),
			'<' => Vector(0,1),
			'v' => Vector(1,1),
			'>' => Vector(2,1),
			_ => panic!(),
		}
	}
	pub fn move_and_get_path(&mut self, c: char) -> Vec<char> {
		let mut path: Vec<char> = vec![];
		let dest = Self::c_to_v(c);
		let diff = dest.sub(&self.posn);
		self.posn = dest;
		// if we have to avoid the space, avoid it
		// otherwise try and create repetitious sequences

		if diff.1 < 0 { 		// if we are moving up, go left/right first then up
			if diff.0 < 0 {
				path.extend(vec!['<'; diff.0.abs() as usize] );
			} else {
				path.extend(vec!['>'; diff.0 as usize] );
			}
			path.extend(vec!['^'; diff.1.abs() as usize])
		} else { 		// if we are moving down, go down first then left/right
			path.extend(vec!['v'; diff.1 as usize]);
			if diff.0 < 0 {
				path.extend(vec!['<'; diff.0.abs() as usize] );
			} else {
				path.extend(vec!['>'; diff.0 as usize] );
			}
		}
		path.push('A');
		path
	}

}


fn path_to_string(path: &Vec<char>) -> String {
	path.into_iter().collect::<String>()
}



pub fn day21(input: &String) -> (String, String) {
	let codes_s: Vec<&str> = input.lines().collect_vec();
	let codes: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
	println!("part 1");
	let mut robot1 = Robot1::new();
	let mut robot2 = RobotM::new();
	let mut robot3 = RobotM::new();
	let mut p1soln: usize = 0;
	for (i,&ref code) in codes.iter().enumerate() {
		let mut path: Vec<char> = vec![];
		for &c0 in code {
			let p1 = robot1.move_and_get_path(c0);
			println!("path1 for {} of length {}: {}", codes_s[i], p1.len(), path_to_string(&p1));
			let p2 = p1.iter().map(|c| robot2.move_and_get_path(*c)).flatten().collect_vec();
			println!("path2 for {} of length {}: {}", codes_s[i], p2.len(), path_to_string(&p2));
			let p3: Vec<char> = p2.iter().map(|c| robot3.move_and_get_path(*c)).flatten().collect_vec();
			println!("path3 for {} of length {}: {}", codes_s[i], p3.len(), path_to_string(&p3));
			path.extend(p3);
		}
		println!("path for {} of length {}: {}", codes_s[i], path.len(), path_to_string(&path));
		let n1 = codes_s[i][0..3].parse::<usize>().unwrap();
		let complexity: usize = path.len() * (codes_s[i][0..3]).parse::<usize>().unwrap();
		println!("complexity {} * {}: {}", n1, path.len(), complexity);
		p1soln += complexity;
	}

	println!("part 1 solution: {}", p1soln);
	(p1soln.to_string(), p1soln.to_string())
}

// <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
// <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A