
use itertools::Itertools;
use std::collections::{*};
#[allow(unused_imports)]
use std::io::stdout;
#[allow(unused_imports)]
use crossterm::{execute,style,style::Stylize};

#[derive(Copy,Clone,Eq,PartialEq,Hash,Ord,PartialOrd)]
struct Array8 {
	pub len: usize,
	pub data: [u8; 8],
}

impl Array8 {
	pub fn from_slice(p: &[u8]) -> Self {
		let mut data: [u8; 8] = [0; 8];
		let len = 8.min(p.len());
		for i in 0..len {
			data[i] = p[i]; 		
		}
		Self {
			data,
			len
		}
	}
	pub fn trunc(&self, s: usize) -> Self {
		let mut data: [u8; 8] = [0; 8];
		let len = 8.min(s);
		for i in 0..len {
			data[i] = self.data[i];
		}
		Self {
			data,
			len
		}				
	}
	pub fn len(&self) -> usize {
		self.len
	}
	#[allow(dead_code)]
	pub fn to_string(&self) -> String {
		let mut s = String::new();
		for i in 0..self.len {
			s += &String::from(self.data[i] as char);
		}
		return s;
	}
}

struct Solver {
	tps: Vec<Array8>,
	length_map: BTreeMap<Array8, u8>,
	pub combo_map: BTreeMap<usize, u64>,
}

impl Solver {
	pub fn new(tps: &Vec<&[u8]>) -> Self {
		let mut nps = tps.iter().map(|&tp| Array8::from_slice(tp)).collect_vec();
		let mut tps_set: BTreeSet<Array8> = BTreeSet::new();
		nps.iter().for_each(|&a| { tps_set.insert(a); });
		nps.sort();		
		Self {
			tps: nps,
			length_map: BTreeMap::new(),		// BTreeMap is faster than HashMap in our case
			combo_map: BTreeMap::new(),
		}
	}
	pub fn lookup(&mut self, a: &Array8) -> bool {
		//self.tps.binary_search(a).is_ok()		// can also try binary_search, if the vec is sorted properly
		self.tps.contains(a)
	}
	pub fn get_lengths(&mut self, a: Array8) -> u8 {
		let mut r: u8 = 0;
		for i in 1..=8 {
			r <<= 1;
			if i <= a.len() {
				let x = self.lookup(&a.trunc(i));
				r |= x as u8;
			}
		}
		return r;
	}
	pub fn cached_get_lengths(&mut self, a: Array8) -> u8 {
		let r = self.length_map.get(&a);
		if let Some(n) = r {
			return *n;
		} else {
			let x = self.get_lengths(a);
			self.length_map.insert(a, x);
			return x;
		}
	}
	pub fn cached_get_num_combos(&mut self, p: &[u8], depth: usize) -> u64 {
		let r = self.combo_map.get(&depth);
		if let Some(n) = r {
			return *n;
		} else {
			let x = self.get_num_combos(p,depth);
			self.combo_map.insert(depth, x);
			return x;
		}
	}
	pub fn get_num_combos(&mut self, p: &[u8], depth: usize) -> u64 {
		let mut r: u64 = 0;
		let a = Array8::from_slice(p);
		let lengths: u8 = self.cached_get_lengths(a);
		for j in 0..=7_usize {
			let len = 8 - j;
			let try_length = ((lengths >> j as u8) & 0x01) != 0;
			if try_length {
				if len == p.len() {
					r += 1;
				} else  if len <= p.len() {
					r += self.cached_get_num_combos(&p[len..], depth+len);
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

pub fn day19(input: &String) -> (String, String) {
	// towel pattern puzzle
	let lines = input.lines().collect_vec();
	let re = regex::Regex::new(r"([rbgwu]+)").expect("valid regex");
	let mut tps = re.find_iter(lines[0]).map(|m| m.as_str().as_bytes()).collect_vec();
	let max_len = tps.iter().max_by(|&&a, &&b| a.len().cmp(&b.len())).unwrap().len();
	let min_len = tps.iter().min_by(|&&a, &&b| a.len().cmp(&b.len())).unwrap().len();
	let patterns = lines[2..].iter().map(|&s| s.as_bytes()).collect_vec();
	tps.sort();

	// print basic stats

	println!("tps: {}", tps.len());
	println!("max tp len: {}", max_len);
	println!("min tp len: {}", min_len);
	println!("patterns: {}", patterns.len());

	println!("part 1 calculating...");

	let mut p1score = 0;
	for &p in patterns.iter() {
		if find_any_soln(p, &tps) {
			p1score += 1;
		}
	}

	println!("part 1 score: {}", p1score);

	// part 2

	println!("part 2 calculating...");

	let mut p2score = 0;
	let mut solver = Solver::new(&tps);
		
	for (_i,&p) in patterns.iter().enumerate() {
		//execute!(stdout(),	style::PrintStyledContent(format!("pattern {}:",i).cyan()),	).unwrap();

		let count: u64 = solver.get_num_combos(p,0);
		solver.combo_map.clear();	// must reset this after every pattern we check

		//execute!(stdout(), style::PrintStyledContent(format!(" {}\n", count).green())).unwrap();
		p2score += count;
	}

	println!("part 2 score: {}", p2score);

	(p1score.to_string(), p2score.to_string())
}