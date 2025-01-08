use itertools::Itertools;
//use std::collections::{*};

pub fn day25(input: &String) -> (String, String) {
	let lines = input.lines().collect_vec();
	let mut idx: usize = 0;
	let mut keys: Vec<[u8;5]> = vec![];
	let mut locks: Vec<[u8;5]> = vec![];
	while idx + 6 < lines.len() {
		let mut deets: [u8; 5] = [0, 0, 0, 0, 0];
		// lock
		for i in 1..=5 {
			for x in 0..=4 {
				if &lines[idx+i][x..=x] == "#" {
					deets[x] = deets[x] + 1;
				}
			}
		}
		if &lines[idx][0..1] == "#" {
			locks.push(deets);
			//println!("found lock: {:?}", deets);
		} else {
			keys.push(deets);
			//println!("found key:  {:?}", deets);
		}
		idx += 8;
	}

	let mut fits = 0;
	for k in keys.iter() {
		for l in locks.iter() {
			let mut result: [u8; 5] = [0; 5];
			let mut failed = false;
			for i in 0..=4 {
				result[i] = l[i] + k[i];
				if result[i] > 5 {
					failed = true;
				}
			}
			if !failed {
				fits += 1;
			}
		}
	}

	println!("part 1 result (fits): {}", fits);

	(fits.to_string(), "no result".to_string())

}