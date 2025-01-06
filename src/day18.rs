//use itertools::Itertools;
//use std::collections::{*};
use crate::grid::{*};
use crate::vector::{*};
use crate::level::{*};
use crate::solve::{*};

use itertools::Itertools;

pub fn day18(input: &String) -> (String, String) {
	// fetch coords
	let re = regex::Regex::new(r"(-?\d+)").expect("valid regex");
	let nums: Vec<i32> = re.find_iter(input).map(|m| m.as_str().parse::<i32>().unwrap()).collect_vec();

	let w = 71;	// ex 7,7
	let h = 71;
	let count = 1024.min(nums.len()/2);

	let mut grid = Grid::new(w,h);
	for i in 0..count {
		let v = Vector(nums[2*i], nums[2*i+1]);
		grid.put_unchecked(&v, b'#');
	}
	grid.put_unchecked(&Vector(0,0), b'S');
	grid.put_unchecked(&Vector(w-1,h-1), b'E');


	let mut level = Level::from_str(&grid.to_string()).unwrap();

	level.start_pos = Vector(0, 0);
	level.end_pos = Vector(w-1, h-1);
	level.deer_pos = Vector(0, 0);

	let soln = find_best_path_18(&level, 1_000);

	let part1_solution = format!("{}", soln.unwrap().score);

	let mut idx_min = count;
	let mut idx_max = nums.len()/2;
	let bmp_cache = level.wall_bmp.clone();

	let mut idx;

	// bisect
	loop {
		idx = (idx_min + idx_max)/2;
		if idx == idx_min {
			//status2 = format!("Found sweet spot min {} max {}", idx_min, idx_max);
			idx = idx_max;
			break;
		}
		level.wall_bmp = bmp_cache.clone();
		for fill_idx in count..=idx {
			let v = Vector(nums[2*fill_idx], nums[2*fill_idx+1]);
			level.wall_bmp.set_v(v);
		}
		let ok = find_any_path_18(&level, 10_000).is_some();

		if ok { idx_min = idx; }
		else { idx_max = idx; }
	}


	let part2_solution = if 2*idx+1 < nums.len() {
		format!("{},{}", nums[2*idx], nums[2*idx+1])
	} else {
		"no solution found".to_string()
	};

	println!("part 1 solution: {}", part1_solution);
	println!("part 2 solution: {}", part2_solution);

	(part1_solution, part2_solution)

}