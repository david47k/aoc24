use itertools::Itertools;
//use std::collections::{*};
//use crate::vector::{*};

use crate::grid::{*};

#[derive(Debug,Clone,Copy)]
struct Robot {
	p: (isize, isize),
	v: (isize, isize),
}

pub fn day14(input: &String) -> (String, String) {
	// read in input to get robot position and velocities
	// NOTE: we've modified input to include the grid size on the first line
	let re = regex::Regex::new(r"(-?\d+)").expect("valid regex");
	let caps: Vec<isize> = re.find_iter(input).map(|m| m.as_str().parse::<isize>().unwrap()).collect_vec();
	let (w,h) = (caps[0], caps[1]);
	println!("w: {w}, h: {h}");

	let robot_desc = &caps[2..caps.len()];
	let mut robots: Vec<Robot> = vec![];
	let robot_count = robot_desc.len() / 4;
	println!("robot count: {}", robot_count);

	for i in 0..robot_count {
		let [px, py, mut vx, mut vy] = robot_desc[i*4..(i+1)*4] else { panic!("invalid robot desc") };
		if vx < 0 {			// keep them positive
			vx = w + vx;
		}
		if vy < 0 {			// keep them positive
			vy = h + vy;
		}
		robots.push(Robot{p: (px,py), v: (vx,vy)});
		//println!("robot: {:?} {:?}", robots[i].p, robots[i].v);
	}

	let mut quads: [usize; 4] = [0; 4];
	let x_midpoint: isize = w / 2;
	let y_midpoint: isize = h / 2;

	for r in &robots {
		// after 100 seconds
		let px = (r.p.0 + r.v.0 * 100) % w;
		let py = (r.p.1 + r.v.1 * 100) % h;
		if px < x_midpoint && py < y_midpoint {
			quads[0] += 1;
		} else if px > x_midpoint && py < y_midpoint {
			quads[1] += 1;
		} else if px < x_midpoint && py > y_midpoint {
			quads[2] += 1;
		} else if px > x_midpoint && py > y_midpoint {
			quads[3] += 1;
		} else {
			// on midpoint, not counted
		}
	}

	let p1_result: usize = quads.iter().product();
	println!("part one result: {}", p1_result);

	// part two
	// now we have to actually look at the grid !!!
	// the pattern for the tree was originally discovered by filtering out times where the
	// middle vertical third of the grid had a much higher density of robots than expected
	// i.e. > 2/3, and scrolling through a few pages of output. the discovered value from
	// that method was too high, but it showed us what to look for!

	let mut p2_result = 0_usize;

	for t in 1..10000 {
		let mut grid = Grid::new_with(w as i32, h as i32, b'.');
		let mut moved_robots = robots.clone();
		for i in 0..robot_count {
			let r = &robots[i];
			moved_robots[i].p.0 = (r.p.0 + r.v.0 * t) % w;
			moved_robots[i].p.1 = (r.p.1 + r.v.1 * t) % h;
			grid.put_unchecked_t(moved_robots[i].p, b'#');
		}
		let s = grid.to_string();
		if s[0..s.len()/2].contains("##########################") {
			println!("=== t={} ===", t);
			println!("{}", grid.to_string());
			p2_result = t as usize;
			break;
		}
		if t % 1000 == 0 {
			println!("--- t={} ---", t);
		}
	}

	(p1_result.to_string(), p2_result.to_string())
}