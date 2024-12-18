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
	let count = 1024;

	let mut grid = Grid::new(w,h);
	for i in 0..count {
		let v = Vector(nums[2*i], nums[2*i+1]);
		grid.put_unchecked(&v, b'#');
	}
	grid.put_unchecked(&Vector(0,0), b'S');
	grid.put_unchecked(&Vector(w-1,h-1), b'E');


	let mut level = Level::from_str(&grid.to_string()).unwrap();
	println!("grid:\n{}", grid.to_string());

	level.start_pos = Vector(0, 0);
	level.end_pos = Vector(w-1, h-1);
	level.deer_pos = Vector(0, 0);

	let soln = find_best_path_18(&level, 1_000);

	println!("{:?}", soln);

	("no result".to_string(), "no result".to_string())

}