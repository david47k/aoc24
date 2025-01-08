use std::collections::VecDeque;
use itertools::Itertools;
//use std::collections::{*};
use crate::grid::{*};
use crate::vector::{*};
use crate::path::{Move};

pub fn day15(input: &String) -> (String, String) {
	let line = &input[0..input.find(&['\n', '\r']).unwrap()];
	let w = line.len();
	let mut data: Vec<String> = vec![];
	let mut lines = input.lines();

	// read in level data
	loop {
		let line = lines.next();
		if line.unwrap().len() == 0 {
			break;
		}
		data.push(line.unwrap().to_string());
	}
	let bdata: Vec<u8> = data.iter().map(|s| s.bytes().collect_vec()).flatten().collect_vec();
	let mut grid = Grid { w: w as i32, h: data.len() as i32, data: bdata.clone() };

	// read in movement data
	let mut movements: Vec<String> = vec![];
	loop {
		let line = lines.next();
		if line.is_none() {
			break;
		}
		movements.push(line.unwrap().to_string());
	}

	let mut moves_m: Vec<Vec<char>> = vec![];
	for s in movements.iter() {
		moves_m.push(s.chars().collect_vec());
	}
	let moves: Vec<Move> = moves_m.iter().flatten().filter(|&&c| c=='^' || c=='>' || c=='v' || c=='<').map(|&c| Move::from_char_unchecked(c)).collect_vec();

	println!("robot moves: {}", moves.len());
	println!("grid w: {}, h: {}, initial position:\n{}", grid.w, grid.h, grid.to_string());

	// find robot
	let mut robot_xy = grid.find(b'@')[0];

	// remove from grid for easier work
	grid.put(&robot_xy, b'.');

	// make move
	for &m in moves.iter() {
		let able = push_boxes(&mut grid, &robot_xy, m);
		if able {
			// move robot
			robot_xy = robot_xy.add(&m.to_vector());
			print!("{} ok. ", m.to_string());
		} else {
			print!("{} failed. ", m.to_string());
		}
	}

	println!("\nfinal position:\n{}", grid.to_string());
	// calculate GPS score -- sum of each box's (100*by+bx)
	let score: i32 = grid.find(b'O').iter().map(|v| v.0 + v.1 * 100).sum();
	println!("part one score: {}", score);



	// part two
	// ddoouubbllee  wwiiddtthh
	// read in level data
	let mut ndata: Vec<u8> = vec![];
	for b in bdata.iter() {
		let dw = match b {
			b'.' => b"..",
			b'O' => b"[]",
			b'@' => b"@.",
			b'#' => b"##",
			_ => panic!("unexpected input byte"),
		};
		ndata.push(dw[0]);
		ndata.push(dw[1]);
	}

	let mut grid = Grid { w: w as i32 * 2, h: (ndata.len() / (w * 2)) as i32, data: ndata };
	println!("\npart two\n");
	println!("grid w: {}, h: {}, initial position:\n{}", grid.w, grid.h, grid.to_string());

	// find robot
	let mut robot_xy = grid.find(b'@')[0];

	// remove from grid for easier work
	grid.put(&robot_xy, b'.');

	// make move
	for (_i,&m) in moves.iter().enumerate() {
		//println!("\nafter {i} moves:\n{}", grid.to_string_with_pt(&robot_xy));
		let nxy = robot_xy.add_dir(&m);
		let nobj = grid.get(&nxy);
		if nobj.is_none() || nobj.unwrap() == b'#' {
			print!("{} failed. ", m.to_string());
			continue;
		}
		let nobj = nobj.unwrap();
		if nobj == b'[' || nobj == b']' {
			let moving_boxes = box_tree_is_pushable(&mut grid, &nxy, m);
			if let Some(mb) = moving_boxes {
				// find new locations
				let nmb = mb.iter().map(|v| v.add_dir(&m)).collect_vec();
				// erase old locations
				mb.iter().for_each(|&v| {
					grid.put(&v, b'.');
					grid.put(&v.add(&Vector(1, 0)), b'.');
				});
				// put boxes in new locations
				nmb.iter().for_each(|&v| {
					grid.put(&v, b'[');
					grid.put(&v.add(&Vector(1, 0)), b']');
				});
				// move robot
				robot_xy = nxy;
				print!("{} ok. ", m.to_string());
				continue;
			}
			print!("{} failed. ", m.to_string());
			continue;
		}
		// if we reach here, should be a .
		// move robot
		robot_xy = robot_xy.add(&m.to_vector());
		print!("{} ok. ", m.to_string());
	}

	println!("\nfinal position:\n{}", grid.to_string_with_pt(&robot_xy));
	// calculate GPS score -- sum of each box's (100*by+bx)
	// NEAREST edge...
	let score2: i64 = grid.find(b'[').iter().map(|v| (v.0 as i64) + v.1  as i64 * 100_i64 ).sum();
	println!("part one score: {}", score);
	println!("part two score: {}", score2);

	(score.to_string(), score2.to_string())
}

fn push_boxes(grid: &mut Grid, xy: &Vector, m: Move) -> bool {	// returns true if move made, false if impossible
	// if we are a wall, fail
	let o = grid.get(&xy);
	if o.is_none() || o == Some(b'#') {
		return false;
	}
	let o = o.unwrap();

	// we should be a box 'O' or a free space (if this is us) '.'

	// what is in the direction we want to move?
	let nxy = xy.add(&m.to_vector());
	let next_o = grid.get(&nxy);
	if next_o.is_none() || next_o == Some(b'#') {
		return false;	// can't move into a wall
	}
	let next_o = next_o.unwrap();
	// next o must be a '.' or an 'O' at this point
	let mut ok = true;
	if next_o == b'O' {
		// try and move next box
		ok = push_boxes(grid, &nxy, m);
	}
	if ok {
		// move whatever we are
		grid.put(&nxy, o);
		grid.put(&xy, b'.');
	}
	ok
}


fn box_tree_is_pushable(grid: &Grid, xyu: &Vector, m: Move) -> Option<Vec<Vector>> {
	// we will save the LEFT side of all the boxes we are pushing, in the vec
	// we are passed in the location of a single box
	// if we are moving left, we check one next position one at a time to look for the .
	// if we are moving right, we can check every 2nd position - i.e. skip the ] positions
	let mut box_list = vec![];
	let mut xy = xyu.clone();
	if grid.get_unchecked(&xy) == b']' {
		xy = Vector(xy.0 - 1, xy.1);
	}
	//println!("btip called with xyu {:?}, xy {:?}, mv {:?}", *xyu, xy, m);
	box_list.push(xy);
	if m == Move::Left || m == Move::Right {
		let mut qxy = xy.add_dir(&m);
		if m == Move::Right {
			qxy = qxy.add_dir(&m);
		}
		loop {
			let qobj = grid.get(&qxy);
			if qobj.is_none() {
				return None;
			}
			let qobj = qobj.unwrap() as char;
			if qobj == '#' {
				return None;
			}
			if qobj == '.' {
				return Some(box_list);
			}
			if qobj == '[' {
				box_list.push(qxy);
				qxy = if m == Move::Right {
					qxy.add_dir(&m).add_dir(&m)
				} else {
					qxy.add_dir(&m)
				};
				continue;
			}
			if qobj == ']' {
				// we should be pushing left
				assert!(m == Move::Left);
				qxy = qxy.add_dir(&m);	// check the next position to the left
				continue;
			}
			panic!("unexpected value {:?} at xy {:?}", qobj, qxy);
		}
	}
	// we are moving up or down
	let mut queue: VecDeque<Vector> = vec![xy.add_dir(&m)].into();
	loop {
		let qxy = queue.pop_front();
		if qxy.is_none() {
			break;
		}
		let qxy = qxy.unwrap();
		let qobj = [ grid.get(&qxy), grid.get(&qxy.add(&Vector(1,0))) ];
		if qobj.iter().any(|o| o.is_none()) {
			return None;
		}
		let qobj = [ qobj[0].unwrap() as char, qobj[1].unwrap() as char ];
		println!("testing {:?} qobjs {:?}", qxy, qobj);
		if qobj.iter().any(|&c| c == '#') {
			return None;
		}
		if qobj.iter().all(|&c| c == '.') {
			//return Some(box_list);
			continue;
		}
		if qobj == [ '[', ']' ] {
			box_list.push(qxy);
			queue.push_back(qxy.add_dir(&m));
			continue;
		}
		if qobj[0] == ']' {
			let z = qxy.add(&Vector(-1,0));
			box_list.push(z);
			queue.push_back(z.add_dir(&m));
		}
		if qobj[1] == '[' {
			let z = qxy.add(&Vector(1,0));
			box_list.push(z);
			queue.push_back(z.add_dir(&m));
		}
	}
	// we survived the queue!
	return Some(box_list);
}
