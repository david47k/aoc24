use itertools::Itertools;
//use std::collections::{*};
//use crate::grid::{*};
//use crate::vector::{*};

use crate::grid::{*};
use crate::vector::{*};

#[derive(Debug,Clone,Copy)]
struct Robot {
	p: (isize, isize),
	v: (isize, isize),
}

pub fn day14(input: &String) -> (usize, usize) {
	// read in input to get robot position and velocities
	// note we've modified input to include the grid size on the first line
	let re = regex::Regex::new(r"(-?\d+)").expect("valid regex");
	let caps: Vec<isize> = re.find_iter(input).map(|m| m.as_str().parse::<isize>().unwrap()).collect_vec();
	let (w,h) = (caps[0], caps[1]);
	println!("w: {w}, h: {h}");
	let robot_desc = &caps[2..caps.len()];
	let mut robots: Vec<Robot> = vec![];
	let robot_count = robot_desc.len() / 4;
	println!("robots: {}", robot_count);
	for i in 0..robot_count {
		let [px, py, mut vx, mut vy] = robot_desc[i*4..(i+1)*4] else { panic!("invalid robot desc") };
		if vx < 0 {			// keep them positive
			vx = w + vx;
		}
		if vy < 0 {			// keep them positive
			vy = h + vy;
		}
		robots.push(Robot{p: (px,py), v: (vx,vy)});
		println!("robot: {:?} {:?}", robots[i].p, robots[i].v);
	}

	// let mut moved_robots: Vec<Robot> = vec![];
	let mut quads: [usize; 4] = [0; 4];
	let x_midpoint: isize = w / 2;
	let y_midpoint: isize = h / 2;
	for r in robots {
		// after 100 seconds
		let px = (r.p.0 + r.v.0 * 100) % w;
		let py = (r.p.1 + r.v.1 * 100) % h;
		println!("   new xy {},{}", px, py);
		// moved_robots.push( Robot{ p: (px,py), v: r.v} );
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

	(p1_result, 0)
}