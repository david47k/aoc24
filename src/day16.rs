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
	let mut best_tiles = BTreeSet::<Vector>::new();
	if soln.len() > 0 {
		println!("{} solutions found", soln.len());
		for s in soln {
			let ss: String = s.path.iter().map(|m| m.to_string()).collect();
			println!("solution : {}", ss);
			best_score = s.score as usize;
			for t in level.get_path_pts(&s.path) {
				best_tiles.insert(t);
			}
		}
		println!("best score: {}", best_score);
		println!("best tiles: {}", best_tiles.len());
	} else {
		println!("no solution");
	}
	(best_score, best_tiles.len())
}