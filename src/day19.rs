
//use std::cmp::min;
use std::io::stdout;
use crossterm::execute;
use itertools::{iproduct, Itertools};
use std::collections::{*};
//use bevy_tasks::futures_lite::stream::try_unfold;
//use crate::grid::{*};
//use crate::vector::{*};
use crossterm::{cursor, style, terminal, queue};
use crossterm::style::Stylize;
use rayon::prelude::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
//use crate::gen_test_data;

struct Solver {
	tps: Vec<[u8]>,
	tps_map: HashMap<[u8], bool>,
	length_map: HashMap<[u8], usize>,

}

// speed consideration:
// - could try copy vs. ref for passing pattern (esp where len <= 8)
// - can store colour in 3 bits

impl Solver {
	pub fn new() -> Solver {
		Self {
			tps: vec![],
			tps_map: HashMap::new(),
			length_map: HashMap::new(),		// HashMap vs BTreeMap... which is faster?!
		}
	}
	pub fn cached_lookup(&mut self, p: &[u8]) -> bool {
		// may be faster with tps.contains or tps_set.contains
		let r = self.tps_map.get(p);
		if let Some(b) = r {
			return *b;
		} else {
			let r = self.tps.contains(p);
			self.tps_map.insert(*p, r);
			return r;
		}
	}
	pub fn get_lengths(&mut self, p: &[u8]) -> u8 {
		let lmax = 8.min(p.len());
		let mut r: u8 = 0;
		for i in 1..=lmax {
			r <<= 1;
			r |= self.cached_lookup(p) as u8;
		}
		return r;
	}
	pub fn cached_get_lengths(&mut self, p: &[u8]) -> u8 {
		let r = self.length_map.get(p);
		if let Some(n) = r {
			return *n;
		} else {
			let r = self.get_lengths(p);
			self.length_map.insert(*p, r);
			return r;
		}
	}
	pub fn get_num_combos(&mut self, p: &[u8]) -> u64 {
		let mut r: u64 = 0;
		for i in 0..p.len() {
			let lengths: u8 = self.cached_get_lengths(p);
			for j in 0..=7_usize {
				let try_length = j as u8 & lengths != 0;
				if try_length {
					if i + j == 0 {
						r += 1;
					} else {
						r += self.get_num_combos(&p[j..]);
					}
				}
			}
		}
		r
	}




}


fn find_tp<'a>(pattern: &[u8], tps: &'a Vec<&[u8]>) -> Vec<&'a &'a [u8]> {
	// returns a list of tp that match part of the pattern
	tps.iter().filter(|&t| pattern.starts_with(t)).collect_vec()
}

// find a valid combinations of tp that produce p
fn find_any_soln(p: &[u8], tps: &Vec<&[u8]>) -> bool {
	if p.len() == 0 {
		return true;		// found matches for all :)
	}
	let vtp = find_tp(p, tps);
	if vtp.len() == 0 {		// no luck
		return false;
	}
	// try and find remaining characters
	for tp in vtp {
		// we've found the first part
		let strp = &p[tp.len()..];
		if find_any_soln(strp, &tps) {
			return true;
		}
	}
	false
}

// same as above but return count of solutions, rather than true/false
fn find_all_soln<'a>(p: &'a [u8], tps: &Vec<&[u8]>) -> u64 {
	if p.len() == 0 {
		return 1;
	}

	let vtp = find_tp(p, tps);
	if vtp.len() == 0 {		// no luck
		return 0;
	}
	// try and find remaining characters
	let mut count: u64 = 0;
	for tp in vtp {
		// we've found the first part
		let strp = &p[tp.len()..];
		count += find_all_soln(strp, &tps);
	}
	count
}

// return number of valid steps from this start position (up to step size 8)
fn get_valid_steps(pattern: &[u8], supermap: &Vec<BTreeMap<&[u8],u64>>) -> Vec<u8> {
	let mut v: Vec<u8> = Vec::with_capacity(8);
	if pattern.len() == 0 {
		return v;
	}
	for len in 1..=7 {
		let c = supermap[len].get(&pattern[0..len]).unwrap();
		if *c != 0 {
			v.push(len as u8);
		}
	}
	v
}

fn test_pattern(pattern: &[u8], supermap: &Vec<BTreeMap<&[u8],u64>>) -> u64 {
	if pattern.len() <= 8 {
		// just use precalculated answer
		return *supermap[pattern.len()].get(pattern).unwrap();
	}

	// find valid steps for this pattern
	let valid_steps = get_valid_steps(pattern, supermap);

	let mut count: u64 = 0;

	// test these steps
	count = valid_steps.par_iter().map(|&s| test_pattern(&pattern[s as usize..], supermap)).sum();

	count
}


pub fn day19(input: &String) -> (String, String) {
	// towel pattern puzzle
	let lines = input.lines().collect_vec();
	let re = regex::Regex::new(r"([rbgwu]+)").expect("valid regex");
	let mut tps = re.find_iter(lines[0]).map(|m| m.as_str().as_bytes()).collect_vec();
	let max_len = tps.iter().max_by(|&&a, &&b| a.len().cmp(&b.len())).unwrap().len();
	let min_len = tps.iter().min_by(|&&a, &&b| a.len().cmp(&b.len())).unwrap().len();
	let patterns = lines[2..].iter().map(|&s| s.as_bytes()).collect_vec();
	tps.sort();
	let pbs: BTreeSet<&[u8]> = BTreeSet::from_iter(tps.clone());		// just a btree of tps

	// print basic stats

	println!("tps: {}", tps.len());
	println!("max tp len: {}", max_len);
	println!("min tp len: {}", min_len);
	println!("patterns: {}", patterns.len());

	// make a supermap of all combos for 0..=8 characters long
	// solve each combo using the slow find_all_soln method

	// println!("building supermap...");
	//
	// let options = [b'r', b'g', b'b', b'w', b'u'];
	// let mut supermap: Vec<BTreeMap<&[u8],u64>> = vec![BTreeMap::new(); 9];
	//
	// let vec0: Vec<&[u8]> = vec![];
	// let vec1: Vec<Vec<u8>> = iproduct!(options.iter()).map(|(&a,)| vec![a]).collect_vec();
	// let vec1: Vec<&[u8]> = vec1.iter().map(|v| &v[..]).collect_vec();
	// let vec2: Vec<Vec<u8>> = iproduct!(options.iter(), options.iter()).map(|(&a,&b)| vec![a,b]).collect_vec();
	// let vec2: Vec<&[u8]> = vec2.iter().map(|v| &v[..]).collect_vec();
	// let vec3: Vec<Vec<u8>> = iproduct!(options.iter(), options.iter(), options.iter()).map(|(&a,&b,&c)| vec![a,b,c]).collect_vec();
	// let vec3: Vec<&[u8]> = vec3.iter().map(|v| &v[..]).collect_vec();
	// let vec4: Vec<Vec<u8>> = iproduct!(options.iter(), options.iter(), options.iter(), options.iter()).map(|(&a,&b,&c,&d)| vec![a,b,c,d]).collect_vec();
	// let vec4: Vec<&[u8]> = vec4.iter().map(|v| &v[..]).collect_vec();
	// let vec5: Vec<Vec<u8>> = iproduct!(options.iter(), options.iter(), options.iter(), options.iter(), options.iter()).map(|(&a,&b,&c,&d,&e)| vec![a,b,c,d,e]).collect_vec();
	// let vec5: Vec<&[u8]> = vec5.iter().map(|v| &v[..]).collect_vec();
	// let vec6: Vec<Vec<u8>> = iproduct!(options.iter(), options.iter(), options.iter(), options.iter(), options.iter(), options.iter()).map(|(&a,&b,&c,&d,&e,&f)| vec![a,b,c,d,e,f]).collect_vec();
	// let vec6: Vec<&[u8]> = vec6.iter().map(|v| &v[..]).collect_vec();
	// let vec7: Vec<Vec<u8>> = iproduct!(options.iter(), options.iter(), options.iter(), options.iter(), options.iter(), options.iter(), options.iter()).map(|(&a,&b,&c,&d,&e,&f,&g)| vec![a,b,c,d,e,f,g]).collect_vec();
	// let vec7: Vec<&[u8]> = vec7.iter().map(|v| &v[..]).collect_vec();
	// let vec8: Vec<Vec<u8>> = iproduct!(options.iter(), options.iter(), options.iter(), options.iter(), options.iter(), options.iter(), options.iter(), options.iter()).map(|(&a,&b,&c,&d,&e,&f,&g,&h)| vec![a,b,c,d,e,f,g,h]).collect_vec();
	// let vec8: Vec<&[u8]> = vec8.iter().map(|v| &v[..]).collect_vec();
	//
	// vec0.iter().for_each(|&s| { let _ = supermap[0].insert(s, find_all_soln(s, &tps)); });
	// vec1.iter().for_each(|&s| { let _ = supermap[1].insert(s, find_all_soln(s, &tps)); });
	// vec2.iter().for_each(|&s| { let _ = supermap[2].insert(s, find_all_soln(s, &tps)); });
	// vec3.iter().for_each(|&s| { let _ = supermap[3].insert(s, find_all_soln(s, &tps)); });
	// vec4.iter().for_each(|&s| { let _ = supermap[4].insert(s, find_all_soln(s, &tps)); });
	// vec5.iter().for_each(|&s| { let _ = supermap[5].insert(s, find_all_soln(s, &tps)); });
	// vec6.iter().for_each(|&s| { let _ = supermap[6].insert(s, find_all_soln(s, &tps)); });
	// println!("building size 7...");
	// vec7.iter().for_each(|&s| { let _ = supermap[7].insert(s, find_all_soln(s, &tps)); });
	// println!("building size 8...");
	// vec8.iter().for_each(|&s| { let _ = supermap[8].insert(s, find_all_soln(s, &tps)); });
	//
	// println!("supermap complete");

	// part 1

	println!("day 19 part 1");

	let mut p1score = 0;
	for &p in patterns.iter() {
		if find_any_soln(p, &tps) {
			p1score += 1;
		}
	}

	println!("part 1 score: {}", p1score);

	// part 2

	println!("day 19 part 2");

	let mut p2score = 0;

	for (i,&p) in patterns.iter().enumerate() {
		execute!(stdout(),
			style::PrintStyledContent(format!("pattern: {}",i).cyan()),
		).unwrap();

		let mut count: u64 = test_pattern(p, &supermap);

		execute!(stdout(), style::PrintStyledContent(format!(" {}\n", count).green())).unwrap();
		p2score += count;
	}

	println!("part 2 score: {}", p2score);

	(p1score.to_string(), p2score.to_string())

}