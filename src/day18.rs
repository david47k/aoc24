//use itertools::Itertools;
//use std::collections::{*};
use crate::grid::{*};
use crate::vector::{*};
use crate::level::{*};
use crate::solve::{*};
use crate::path2::ShrunkPath;

use itertools::Itertools;

pub fn day18(input: &String) -> (String, String) {
	clear_screen();

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
	let mut status1: String = "Part 1 processing".to_string();
	let mut status2: String = "".to_string();
	visualise(&level, None, &status1, &status2);

	level.start_pos = Vector(0, 0);
	level.end_pos = Vector(w-1, h-1);
	level.deer_pos = Vector(0, 0);

	let soln = find_best_path_18(&level, 1_000, Some(callback));

	let part1_solution = format!("{}", soln.unwrap().score);
	status2 = format!("Part 1 solution: {}", part1_solution);
	status1 = "Part 2 processing...".to_string();
	visualise(&level, None, &status1, &status2);

	let mut idx_min = count;
	let mut idx_max = nums.len()/2;
	let bmp_cache = level.wall_bmp.clone();

	let mut idx;

	// bisect
	loop {
		idx = (idx_min + idx_max)/2;
		if idx == idx_min {
			status2 = format!("Found sweet spot min {} max {}", idx_min, idx_max);
			idx = idx_max;
			break;
		}
		level.wall_bmp = bmp_cache.clone();
		for fill_idx in count..=idx {
			let v = Vector(nums[2*fill_idx], nums[2*fill_idx+1]);
			level.wall_bmp.set_v(v);
		}
		let ok = find_any_path_18(&level, 10_000).is_some();
		status2 = if ok { format!("Idx {} ok", idx) }
		else { format!("Idx {} fail", idx) };

		if ok { idx_min = idx; }
		else { idx_max = idx; }

		visualise(&level, None, &status1, &status2);
	}

	let part2_solution = format!("{},{}", nums[2*idx], nums[2*idx+1]);
	status2 = format!("Part 2 solution: {} (idx {})", part2_solution, idx);
	visualise(&level, None, &status1, &status2);

	(part1_solution, part2_solution)

}

use std::io::{stdout, Write};
use std::time;
use crossterm::{cursor, style, terminal, queue};
use crossterm::style::Stylize;

fn callback(level: &Level, path: &ShrunkPath, depth: u64) {
	// call visualise
	visualise(level, Some(path), &"Day 18 part 1:".to_string(), &format!("Depth: {}", depth));
}

fn clear_screen() {
	let mut stdout = stdout();
	queue!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
	stdout.flush().unwrap();
}
// visualise
fn visualise(level: &Level, path: Option<&ShrunkPath>, status1: &String, status2: &String) {
	let mut stdout = stdout();
	//  terminal::Clear(terminal::ClearType::All)
	queue!(stdout, cursor::MoveTo(0, 0), terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
	queue!(stdout, style::PrintStyledContent(status1.clone().yellow())).unwrap();
	let level_str = level.to_string();
	// add path to level_str... soln.unwrap().path -> coords
	queue!(stdout, cursor::MoveTo(0, 1)).unwrap();
	queue!(stdout, style::Print(level_str)).unwrap();
	queue!(stdout, terminal::Clear(terminal::ClearType::CurrentLine), style::PrintStyledContent(status2.clone().yellow())).unwrap();
	stdout.flush().unwrap();
	std::thread::sleep(time::Duration::from_millis(100));
}

