//use itertools::Itertools;
use crate::level::{*};
use crate::solve::{*};

pub fn day16(input: &String) -> (String, String) {
	let level = Level::from_str(input).expect("valid level");
	let mut best_score: usize = 0;
	println!("level w: {}, h: {}", level.w, level.h);
	println!("part 1 & 2 calculating...");
	//println!("{}", level.to_string());
	let soln = find_best_path_16(&level, 1_000);
	let mut best_tiles: usize = 0;
	if let Some(sol) = soln {
		//println!("Solution found!");
		//let ss: String = sol.path.iter().map(|m| m.to_string()).collect();
		//println!("solution : {}", ss);

		best_score = sol.score as usize;
		best_tiles = sol.visited.len();

		println!("best score: {}", best_score);
		println!("best tiles: {}", best_tiles);
	} else {
		println!("no solution");
	}
	(best_score.to_string(), best_tiles.to_string())
	// ex16b.txt
	// best score: 11048
	// best tiles: 64

	// ex16.txt
	// best score: 7036
	// best tiles: 45

}