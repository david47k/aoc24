
use crate::level::{Level};
use crate::vector::{*};
use crate::path2::{Move2,ShrunkPath,ALLMOVES2};

use std::collections::{BTreeMap};
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Solution {
	pub score: u64,
	pub path: Vec<Move2>,
	pub visited: Vec<Vector>,
	pub max_depth_hit: bool,
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

pub fn find_best_path_16(level: &Level, max_depth: u64) -> Option<Solution> {
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
		//println!("depth: {}", depth);
		// for each edgenode
		for (id,data) in edge_nodes.iter() {
			//println!("testing edgenode {:?} with score {}", id, data.s);

			// is it a winner? save it if so
			if id.p == level.end_pos {
				//println!("\nsolution found with score {} at depth {}", data.s, depth);
				//println!("path: {}", data.path.to_string());
				if solutions.len() == 0 || data.s < solutions[0].1.s {
					solutions = [(*id, data.clone())].to_vec();
					println!("best solution so far found with score {} at depth {}", data.s, depth);
				} else if data.s == solutions[0].1.s {
					solutions.push((*id, data.clone()));
					//println!("--> additional best solution!");
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
		Some( Solution { score, path: path.to_path(), visited: pts, max_depth_hit: false } ) // max_depth_hit not implemented
	} else {
		None
	}
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
struct NodeID18 {
	pub p: Vector,    // deer position
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
struct NodeData18 {
	pub s: u64,				// score
	pub path: ShrunkPath,	// path to reach this point
}

pub fn find_best_path_18(level: &Level, max_depth: u64) -> Option<Solution> {
	let mut depth: u64 = 0;
	let mut max_depth_hit = false;
	let mut nodes: BTreeMap<NodeID18,NodeData18> = BTreeMap::new();
	let mut edge_nodes: Vec<(NodeID18, NodeData18)> = vec![];
	let mut extra_nodes: Vec<(NodeID18, NodeData18)> = vec![];
	let mut solutions: Vec<(NodeID18, NodeData18)> = vec![];
	let first_node_id = NodeID18 { p: level.deer_pos };
	let first_node_data = NodeData18 { s: 0, path: ShrunkPath::new() };
	nodes.insert(first_node_id, first_node_data.clone() );
	edge_nodes.push((first_node_id, first_node_data ));
	while edge_nodes.len() > 0 && depth < max_depth {
		// for each edgenode
		for (id,data) in edge_nodes.iter() {
			// is it a winner? save it if so
			if id.p == level.end_pos {
				//println!("\nsolution found with score {} at depth {}", data.s, depth);
				//println!("path: {}", data.path.to_string());

				if solutions.len() == 0 || data.s < solutions[0].1.s {
					solutions = [(*id, data.clone())].to_vec();
					//println!("--> best solution so far!");
				} else if data.s == solutions[0].1.s {
					solutions.push((*id, data.clone()));
					//println!("--> additional best solution!");
				}
			}

			// get directions -- vector, object, score
			let maybes = ALLMOVES2.iter().map(|&m| (m, id.p.apply_dir(&m)));
			let maybes = maybes.filter(|(_m, p)| level.has_space_at(*p) );
			let maybes = maybes.collect_vec();
			let maybes = maybes.into_iter().map(|(m, p)| (m, p, data.s + 1));                // move, pos, score
			let maybes = maybes.collect_vec();

			// remove any from our list that have better contenders in nodes
			let maybes = maybes.into_iter().filter(|(m, p, s)| {
				let key = NodeID18 { p: *p };
				if let Some(existing_data) = nodes.get_mut(&key) {
					if *s < existing_data.s {
						// lower score is better! override any existing data
						let mut path = data.path.clone();
						path.push(m);
						let val = NodeData18 { s: *s, path: path };	// any path is OK
						//nodes.insert(key, val.clone());			// update it with our awesome data
						existing_data.path = path;
						existing_data.s = *s;
						extra_nodes.push((key, val));
					}
					return false;    // we've handled it
				} else {
					return true;    // keep it,we haven't seen it before
				}
			}).collect_vec();

			// insert the new ones
			for (m, p, s) in maybes.into_iter() {
				let key = NodeID18 { p: p };
				let mut path = data.path.clone();
				path.push(&m);
				let val = NodeData18 { s: s, path };
				nodes.insert(key, val.clone());
				extra_nodes.push((key, val));
			}
		}
		edge_nodes = extra_nodes.clone();
		extra_nodes.clear();
		depth += 1;
		if depth >= max_depth {
			max_depth_hit = true;
		}
	}
	if solutions.len() > 0 {
		let path = solutions[0].1.path;
		let score = solutions[0].1.s;
		let pts = level.get_path_pts(&path.to_path()).clone();
		Some( Solution { score, path: path.to_path(), visited: pts, max_depth_hit } )
	} else {
		if depth==max_depth {
			//println!("WARNING: hit max depth!");
		}
		None
	}
}


pub fn find_any_path_18(level: &Level, max_depth: u64) -> Option<Solution> {
	let mut depth: u64 = 0;
	let mut max_depth_hit = false;
	let mut nodes: BTreeMap<NodeID18,NodeData18> = BTreeMap::new();
	let mut edge_nodes: Vec<(NodeID18, NodeData18)> = vec![];
	let mut extra_nodes: Vec<(NodeID18, NodeData18)> = vec![];
	let mut solutions: Vec<(NodeID18, NodeData18)> = vec![];
	let first_node_id = NodeID18 { p: level.deer_pos };
	let first_node_data = NodeData18 { s: 0, path: ShrunkPath::new() };
	nodes.insert(first_node_id, first_node_data.clone() );
	edge_nodes.push((first_node_id, first_node_data ));
	while edge_nodes.len() > 0 && depth < max_depth {
		// for each edgenode
		for (id,data) in edge_nodes.iter() {
			// is it a winner? save it if so
			if id.p == level.end_pos {
				if solutions.len() == 0 || data.s < solutions[0].1.s {
					solutions = [(*id, data.clone())].to_vec();
					//println!("--> best solution so far!");
					depth = max_depth;	// quit
					break;
				}
			}

			// get directions -- vector, object, score
			let maybes = ALLMOVES2.iter().map(|&m| (m, id.p.apply_dir(&m)));
			let maybes = maybes.filter(|(_m, p)| level.has_space_at(*p) );
			let maybes = maybes.collect_vec();
			let maybes = maybes.into_iter().map(|(m, p)| (m, p, data.s + 1));                // move, pos, score
			let maybes = maybes.collect_vec();

			// remove any from our list that have better contenders in nodes
			let maybes = maybes.into_iter().filter(|(m, p, s)| {
				let key = NodeID18 { p: *p };
				if let Some(existing_data) = nodes.get_mut(&key) {
					if *s < existing_data.s {
						// lower score is better! override any existing data
						let mut path = data.path.clone();
						path.push(m);
						let val = NodeData18 { s: *s, path: path };	// any path is OK
						//nodes.insert(key, val.clone());			// update it with our awesome data
						existing_data.path = path;
						existing_data.s = *s;
						extra_nodes.push((key, val));
					}
					return false;    // we've handled it
				} else {
					return true;    // keep it,we haven't seen it before
				}
			}).collect_vec();

			// insert the new ones
			for (m, p, s) in maybes.into_iter() {
				let key = NodeID18 { p: p };
				let mut path = data.path.clone();
				path.push(&m);
				let val = NodeData18 { s: s, path };
				nodes.insert(key, val.clone());
				extra_nodes.push((key, val));
			}
		}
		edge_nodes = extra_nodes.clone();
		extra_nodes.clear();
		depth += 1;
		if depth >= max_depth {
			max_depth_hit = true;
		}
	}
	if solutions.len() > 0 {
		let path = solutions[0].1.path;
		let score = solutions[0].1.s;
		Some( Solution { score, path: path.to_path(), visited: vec![], max_depth_hit } )
	} else {
		if depth==max_depth {
			//println!("WARNING: hit max depth!");
		}
		None
	}
}