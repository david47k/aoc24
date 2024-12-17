// boxboppertool Copyright 2020-2021 David Atkinson
//
// solve.rs: solve a sokoban-style level

use crate::level::{Level,CmpData};
use crate::time::{get_time_ms};
use crate::vector::{*};
use crate::path2::{Move2,ShrunkPath,ALLMOVES2};
use crate::sprite::Obj;

use rayon::prelude::*;
use std::rc::Rc;
use std::collections::{BTreeMap};
use std::cmp::Ordering;
use itertools::Itertools;
use crate::pathnodemap::{PathMap};

use bevy_tasks::{TaskPool,TaskPoolBuilder};

pub fn task_splitter(pool: &TaskPool, spl_into: usize, from: &Vec::<PathMap>, func: impl Fn(&[PathMap], &mut Vec::<PathMap>) + Send + Copy + Sync) -> Vec::<PathMap> {
	// break up vecs
	let from_a = vec_slicer(from, spl_into);
	let mut to_a = vec_new_split_store(from.len() / spl_into + 1, spl_into);

	pool.scope(|s| {
		for i in 0..spl_into {
			unsafe { // actually safe, as we don't use overlapping indices
				let from_sm: &[PathMap] = *(from_a.get_unchecked(i) as *const _);
				let to_sm = &mut *(to_a.get_unchecked_mut(i) as *mut _);
				s.spawn( async move {
					func(&from_sm, to_sm);
				})
			}
		}
	});

	let maps = vec_unslice(to_a);
	maps
}

pub fn task_splitter_mut(pool: &TaskPool, spl_into: usize, mut maps: Vec::<PathMap>, func: impl Fn(&mut [PathMap]) + Send + Copy + Sync) -> Vec::<PathMap> {
	// break up vecs
	let mut maps_a = vec_slicer_mut(&mut maps, spl_into);

	pool.scope(|s| {
		for i in 0..spl_into {
			unsafe { // actually safe, as we don't use overlapping indices
				let maps_sm: &mut &mut [PathMap] = &mut *(maps_a.get_unchecked_mut(i) as *mut _);
				s.spawn( async move {
					func(maps_sm);
				})
			}
		}
	});

	// no unslice required, as we used mutable references :)
	maps
}

// Faster than rayon::par_sort_unstable
pub fn task_splitter_sort(pool: &TaskPool, spl_into: usize, mut maps: Vec::<PathMap>) -> Vec::<PathMap> {
	// break up vecs
	let mut maps_a = vec_slicer_mut(&mut maps, spl_into);

	pool.scope(|s| {
		for i in 0..spl_into {
			unsafe { // actually safe, as we don't use overlapping indices
				let maps_sm: &mut &mut [PathMap] = &mut *(maps_a.get_unchecked_mut(i) as *mut _);
				s.spawn( async move {
					maps_sm.sort_unstable_by(|a: &PathMap, b: &PathMap| {
						let ord = a.level.cmp_data.partial_cmp(&b.level.cmp_data).unwrap();
						if ord == Ordering::Equal {
							if a.path.len() < b.path.len() {
								return Ordering::Less;
							}
							if a.path.len() > b.path.len() {
								return Ordering::Greater;
							}
						}
						ord
					})
				})
			}
		}
	});

	fn pm_cmp_lt (a: & &mut PathMap, b: & &mut PathMap) -> bool {
		let ord = a.level.cmp_data.partial_cmp(&b.level.cmp_data).unwrap();
		if ord == Ordering::Equal {
			if a.path.len() < b.path.len() {
				return true;
			}
			return false;
		}
		return ord == Ordering::Less;
	}

	let mut maps: Vec::<PathMap> = maps_a.into_iter().map(|x| x).kmerge_by(pm_cmp_lt).map(|x| x.to_owned()).collect();
	maps.dedup_by(|a,b| a.level.cmp_data == b.level.cmp_data); // it keeps the first match for each level (sorted to be smallest moves)
	maps
}

// borrows the provided vec, and provides a vec of slices as output
pub fn vec_slicer(from: &Vec::<PathMap>, spl_into: usize) -> Vec::<&[PathMap]> {
	let size = from.len() / spl_into;
	let mut out: Vec::<&[PathMap]>;
	out = Vec::<&[PathMap]>::with_capacity(spl_into);
	if from.len() < spl_into { 
		out.push( &from[..] );
		for _i in 1..spl_into  {
			out.push( &from[0..0] );
		}
	} else {
		let mut count = 0;
		for _i in 1..spl_into  {
			out.push( &from[count..(count+size)] );
			count += size;
		}
		out.push( &from[count..] );
	}
	out
}

// borrows the provided vec, and provides a vec of mutable slices as output
pub fn vec_slicer_mut(from: &mut Vec::<PathMap>, spl_into: usize) -> Vec::<&mut [PathMap]> {
	// assert(spl_into > 0);
	let size = from.len() / spl_into;
	let mut out: Vec::<&mut [PathMap]> = Vec::<&mut [PathMap]>::with_capacity(spl_into);
	unsafe { // actually safe, as we don't use overlapping indices
		if from.len() < spl_into { 
			let from_sm: &mut [PathMap] = &mut *(from.get_unchecked_mut(..) as *mut _);
			out.push( from_sm );
			for _i in 1..spl_into  {
				let from_sm: &mut [PathMap] = &mut *(from.get_unchecked_mut(0..0) as *mut _);
				out.push( from_sm );
			}
		} else {
			let mut count = 0;
			for _i in 1..spl_into  {
				let from_sm: &mut [PathMap] = &mut *(from.get_unchecked_mut(count..(count+size)) as *mut _);
				out.push( from_sm );
				count += size;
			}
			out.push( &mut from[count..] );
		}
	}
	out
}

pub fn vec_new_split_store(size: usize, spl_into: usize) -> Vec::<Vec::<PathMap>> {
	let mut out = Vec::<Vec::<PathMap>>::with_capacity(spl_into);
	for _i in 0..spl_into  {
		out.push( Vec::<PathMap>::with_capacity(size) );
	}
	out
}

pub fn vec_unslice(mut from: Vec::<Vec::<PathMap>>) -> Vec::<PathMap> {
	for i in 1..from.len() {
		unsafe {
			// actually safe because we aren't accessing two identical indicies (it'll always be 0 and 1+)
			let src = &mut *(from.get_unchecked_mut(i) as *mut _);
			from[0].append(src);
		}
	}
	return from.swap_remove(0);
}

#[derive(Clone, Debug)]
pub struct Solution {
	pub score: u64,
	pub path: Vec<Move2>,
}
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
struct NodeID {
	pub p: Vector,    // deer position
	pub d: Move2,    // deer direction
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
struct NodeData {
	pub s: u64,			// score
	pub path: ShrunkPath,	// path to reach this point
}

pub fn find_best_path(level: &Level, max_score: u64) -> Vec<Solution> {
	let max_depth = 1000;
	// find path from start_pos to end_pos
	// using score as path weight
	// we will store how we get to each square:
	//   (position, direction)   point identifier (key)
	//   (score)                 point weight
	//   (path as shrunkpath)    path to use
	// this way we can sort & choose optimum path
	// we haven't been in a location unless it's the same position AND direction!
	// we keep going until we can't find any more paths to visit
	// we keep seperate track of the best path that reach the finish line
	// we will keep track of depth --- so we don't go too deep
	let mut depth: u64 = 0;
	let mut nodes: BTreeMap<NodeID,NodeData> = BTreeMap::new();
	let mut edge_nodes: Vec<(NodeID, NodeData)> = vec![];
	let mut extra_nodes: Vec<(NodeID, NodeData)> = vec![];
	let mut solutions: Vec<(NodeID, NodeData)> = vec![];
	let first_node_id = NodeID { p: level.deer_pos, d: level.deer_dir };
	let first_node_data = NodeData { s: 0, path: ShrunkPath::new() };
	nodes.insert(first_node_id, first_node_data );
	edge_nodes.push((first_node_id, first_node_data ));
	while edge_nodes.len() > 0 && depth < max_depth {
		println!("depth: {}", depth);
		// for each edgenode
		for (id,data) in edge_nodes.iter() {
			//println!("testing edgenode {:?} with score {}", id, data.s);
			// is it a winner? save it if so
			if id.p == level.end_pos {
				println!("\nsolution found with score {} at depth {}", data.s, depth);
				println!("path: {}", data.path.to_string());
				if solutions.len() == 0 || data.s < solutions[0].1.s {
					solutions = [(*id, *data)].to_vec();
					println!("--> best solution so far!");
				} else if data.s == solutions[0].1.s {
					solutions.push((*id, *data));
					println!("--> additional best solution!");
				}
			}

			// get directions -- vector, object, score
			let mut maybes = ALLMOVES2.iter().map(|&m| (m, id.p.apply_dir(&m)));
			let mut maybes = maybes.filter(|(m, p)| level.has_space_at(*p) );
			let mut maybes = maybes.collect_vec();
			let mut maybes = maybes.into_iter().map(|(m, p)| (m, p, data.s + ShrunkPath::calc_score(&m, &id.d)));                // move, pos, score
			let mut maybes = maybes.collect_vec();
			// we now know mps: Move, Pos, Score

			//println!("{} directions found", maybes.len());

			// remove any from our list that have better/same contenders in nodes
			maybes = maybes.into_iter().filter(|(m, p, s)| {
				let key = NodeID { p: *p, d: *m };
				if let Some(existing_data) = nodes.get(&key) {
					if *s < existing_data.s {
						// lower score is better
						let mut path = data.path.clone();
						path.push(m);

						let val = NodeData { s: *s, path };
						nodes.insert(key, val);
						extra_nodes.push((key, val));
					}
					return false;    // we've handled it
				} else {
					return true;    // keep it,we haven't seen it before
				}
			}).collect_vec();

			// insert the new ones
			for (m, p, s) in maybes.into_iter() {
				let key = NodeID { p: p, d: m };
				let mut path = data.path.clone();
				path.push(&m);
				let val = NodeData { s: s, path };
				nodes.insert(key, val);
				extra_nodes.push((key, val));
			}
		}
		edge_nodes = extra_nodes.clone();
		extra_nodes.clear();
		depth += 1;
	}
	solutions.iter().map(|(id,data)| Solution { score: data.s, path: data.path.to_path() } ).collect_vec()
}

pub fn solve_level(base_level_in: &Level, max_score_requested: u64, max_maps: usize, verbosity: u32, num_threads: usize) -> Option<Solution> {
	let mut max_score = max_score_requested+1;
	let base_level1 = base_level_in;
	let base_map = PathMap::new_from_level(&base_level1);
	let base_level = base_level1.clone();	// !
	let mut non_contenders = BTreeMap::<CmpData,u64>::new();

	let mut bvec = Vec::new();
	bvec.push(base_map);
	let mut mapsr = Rc::new(bvec);
	
	let pool = TaskPoolBuilder::new()
		.thread_name("Box Bopper Tool Thread Pool".to_string())
		.num_threads(num_threads)
		.build();

	let mut have_solution = false;
	struct BestSolution {
		s: Vec<Move2>,
		score: u64,
	}
	let mut best_solution = BestSolution { s: Vec::<Move2>::new(), score: 0 };
	let mut depth: u64 = 0;
	let max_depth: u64 = 1000;
	
	let msecs0 = get_time_ms();

	while (depth as u64) < max_score {
		if verbosity > 0 { println!("-- Depth {:>2} --", depth); }

		// Check for level complete / having solution
		if verbosity > 1 { println!("solution check..."); }
		mapsr.iter().filter(|m| m.level.have_win_condition(&base_level)).for_each(|m| {
			if m.path.score() < max_score {
				have_solution = true;
				max_score = m.path.score();
				best_solution.score = m.path.score();
				best_solution.s = m.path.to_path();
				if verbosity > 0 { 
					println!("-- Solution found in {} moves --", m.path.len());
				}
			}
		});

		// We have to store number of moves, because higher depth can have less moves
		if verbosity > 1 { println!("adding {} old maps to non-contenders...", mapsr.len()); }
		if non_contenders.len() < max_maps * 4 {
			//mapsr.par_iter().for_each(|m| { non_contenders.insert(m.level.cmp_data, m.path.len()); });
			non_contenders.par_extend(mapsr.par_iter().map(|m| (m.level.cmp_data, m.path.score()) ));
		} else {
			if verbosity > 0 { println!("--- Old maps hit max_maps limit, not adding more ---"); }				// Performance will drag after this point, as we'll probably end up repeating moves
		}

		// Perform next key moves
		if verbosity > 1 { println!("performing next key moves..."); }
		let mut maps = task_splitter(&pool, num_threads, &mapsr, |maps_read: &[PathMap], mut maps_write: &mut Vec::<PathMap>| {
			maps_read.iter().for_each(|m| m.complete_solve_2(&base_level, &mut maps_write));		// perform next key moves
			maps_write.retain(|m| m.path.score() < max_score);										// filter out long moves
		});

		// Sort and deduplicate
		if depth >= 2 { 
			if verbosity > 1 { println!("deduping: before {:>7}", maps.len()); }
			maps = task_splitter_sort(&pool, num_threads, maps);
			if verbosity > 1 { println!("deduping: after  {:>7}", maps.len()); }
		} 

		// Remove from maps anything that is in non_contenders AND our path is equal/longer. (Our shorter paths will be updated/added at the next loop)
		if verbosity > 1 { println!("deduping using n-c: before {:>7}", maps.len()); }		
		maps = task_splitter_mut(&pool, num_threads, maps, |maps: &mut [PathMap]| {
			for m in maps {
				let v = non_contenders.get(&m.level.cmp_data);
				if v.is_some() {
					if *v.unwrap() <= m.path.score() {
						m.flag = true;
					}
				}
			}
		});
		maps.retain(|m| !m.flag);
		if verbosity > 1 { println!("deduping using n-c: after  {:>7}", maps.len()); }

		// Check if we've exhausted the search space
		if maps.len() == 0 {
			if verbosity > 0 { println!("-- No more maps to check --"); }
			break;
		}

		// Check if we've hit max_maps (our memory/resource limit)
		if maps.len() > max_maps {
			println!("--- Hit maximum maps ({}) ---",max_maps);
			println!("--- Purging lots of maps, solutions may be thrown out ---");
			maps.truncate(max_maps/2);
		}
		
		// Loop and check the next depth
		mapsr = Rc::new(maps);
		depth += 1;
	}

	if have_solution {
		let sol = best_solution;		
		if verbosity > 0 { 
			println!("-- Best solution --");
			println!("Solution with score: {}", max_score);
		}
		return Some(Solution {
			score: max_score,
			path: sol.s,
		});
	} else {
		let ms = max_score;
		if verbosity > 0 {
			println!("-- No solution found --");
			if ms > 1 { println!("Max moves was {}",ms-1); }
		}
		return None;
	}

}