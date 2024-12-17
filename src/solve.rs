// boxboppertool Copyright 2020-2021 David Atkinson
//
// solve.rs: solve a sokoban-style level

use crate::level::{Level};
use crate::vector::{*};
use crate::path2::{Move2,ShrunkPath,ALLMOVES2};

//use rayon::prelude::*;
//use std::rc::Rc;
use std::collections::{BTreeMap};
use std::cmp::Ordering;
use itertools::Itertools;
use crate::pathnodemap::{PathMap};

use bevy_tasks::{TaskPool}; //,TaskPoolBuilder};

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
	pub visited: Vec<Vector>,
}
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
struct NodeID {
	pub p: Vector,    // deer position
	pub d: Move2,    // deer direction
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
struct NodeData {
	pub s: u64,				// score
	pub path: ShrunkPath,	// path to reach this point
	pub pts: Vec<Vector>,
}

pub fn find_best_path(level: &Level, max_depth: u64) -> Option<Solution> {
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
	let first_node_data = NodeData { s: 0, path: ShrunkPath::new(), pts: [level.deer_pos].to_vec() };
	nodes.insert(first_node_id, first_node_data.clone() );
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
					solutions = [(*id, data.clone())].to_vec();
					println!("--> best solution so far!");
				} else if data.s == solutions[0].1.s {
					solutions.push((*id, data.clone()));
					println!("--> additional best solution!");
				}
			}

			// get directions -- vector, object, score
			let maybes = ALLMOVES2.iter().map(|&m| (m, id.p.apply_dir(&m)));
			let maybes = maybes.filter(|(_m, p)| level.has_space_at(*p) );
			let maybes = maybes.collect_vec();
			let maybes = maybes.into_iter().map(|(m, p)| (m, p, data.s + ShrunkPath::calc_score(&m, &id.d)));                // move, pos, score
			let maybes = maybes.collect_vec();
			// we now know mps: Move, Pos, Score

			//println!("{} directions found", maybes.len());

			// remove any from our list that have better contenders in nodes
			let maybes = maybes.into_iter().filter(|(m, p, s)| {
				let key = NodeID { p: *p, d: *m };
				if let Some(existing_data) = nodes.get_mut(&key) {
					if *s < existing_data.s {
						// lower score is better! override any existing data
						let mut path = data.path.clone();
						path.push(m);
						existing_data.pts = level.get_path_pts(&path.to_path());
						let val = NodeData { s: *s, path: path, pts: existing_data.pts.clone() };	// any path is OK
						//nodes.insert(key, val.clone());			// update it with our awesome data
						existing_data.path = path;
						existing_data.s = *s;
						extra_nodes.push((key, val));
					} else if *s == existing_data.s {
						// this is an ALTERNATE way of getting to this point
						let mut path = data.path.clone();
						path.push(m);

						// add the points from our list, and dedup
						// SLOW POINT. Definitely not the fastest way !!!!
						let mut extra_pts = level.get_path_pts(&path.to_path());
						if extra_pts.iter().any(|xp| !existing_data.pts.contains(xp)) {
							existing_data.pts.append(&mut extra_pts);
							existing_data.pts.sort();
							existing_data.pts.dedup();
							let val = NodeData { s: *s, path, pts: existing_data.pts.clone() };
							extra_nodes.push((key, val));
						}
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
				let val = NodeData { s: s, path, pts: level.get_path_pts(&path.to_path()) };
				nodes.insert(key, val.clone());
				extra_nodes.push((key, val));
			}
		}
		edge_nodes = extra_nodes.clone();
		extra_nodes.clear();
		depth += 1;
	}
	if solutions.len() > 0 {
		let path = solutions[0].1.path;
		let score = solutions[0].1.s;
		let pts = nodes.get(&solutions[0].0).unwrap().pts.clone();
		//let paths = solutions.iter().map(|(_, data)| data.path.iter().map(|p| p.to_path()).collect_vec()).flatten().collect_vec();
		Some( Solution { score, path: path.to_path(), visited: pts } )
	} else {
		None
	}
}

