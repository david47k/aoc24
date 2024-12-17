//use itertools::Itertools;
use std::collections::{*};
use crate::grid::{*};
use crate::vector::{*};
use crate::level::{*};
use crate::solve::{*};

pub fn day16(input: &String) -> (usize, usize) {
	let level = Level::from_str(input).expect("valid level");
	let mut best_score: usize = 0;
	println!("level w: {}, h: {}", level.w, level.h);
	println!("{}", level.to_string());
	let soln = find_best_path(&level, 1_000_000);
	let mut best_tiles: usize = 0;
	if let Some(sol) = soln {
		println!("Solution found!");
		let ss: String = sol.path.iter().map(|m| m.to_string()).collect();
		println!("solution : {}", ss);

		best_score = sol.score as usize;
		best_tiles = sol.visited.len();

		println!("best score: {}", best_score);
		println!("best tiles: {}", best_tiles);
	} else {
		println!("no solution");
	}
	(best_score, best_tiles)
}