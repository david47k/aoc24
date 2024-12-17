// boxboppertool Copyright 2020-2021 David Atkinson
//
// pathnodemap.rs: PathNode, PathMap, PathNodeMap and family
// Used for creating and solving levels

use crate::sprite::{Obj};
use crate::level::{Level,SpLevel,CmpData};
use crate::vector::{Vector};
use crate::path::{ALLMOVES};
use crate::path2::{Move2,ShrunkPath};
use crate::stackstack::{StackStack16,StackStack8};

#[derive(Clone,Copy)]
pub struct PathNode {
	pt: Vector,
	prev_node_idx: u16,
	move_taken: Option<Move2>, // what move we took to get here, used to determine movelist when solution found
}

#[derive(Clone,Copy)]
pub struct KeyMove2 {
	pni: u16,			// where deer is just before pushing boxx - pathnode index
	move_dir: Move2,		// direction to move to push boxx (or direction we are pulling box in)
}

#[derive(Clone)]
pub struct PathNodeMap {
	pub nodes: Vec::<PathNode>,
	pub key_moves: Vec::<KeyMove2>,	
}

#[derive(Clone)]
pub struct PathMap {
	pub level: SpLevel,
	pub path: ShrunkPath,
	pub score: u64,
	pub flag: bool,
}

impl PathMap {
	pub fn new() -> PathMap {
		PathMap {
			level: SpLevel {
				w: 0,
				h: 0,
				cmp_data: CmpData::new(),
			},
			path: ShrunkPath::new(),
			score: 0,
			flag: false,
		}
	}
	pub fn new_from_level(level: &Level) -> PathMap {
		PathMap {
			level: SpLevel::from_level(level),
			path: ShrunkPath::new(),
			score: 0,
			flag: false,
		}
	}
	pub fn to_pnm(&self) -> PathNodeMap {		// this one clones across our data
		let initial_pn = PathNode {
			pt: Vector(self.level.cmp_data.deer_x as i32, self.level.cmp_data.deer_y as i32),
			move_taken: None,
			prev_node_idx: 0,
		};
		let mut nodes = Vec::<PathNode>::with_capacity(256/(std::mem::size_of::<PathNode>()));
		nodes.push(initial_pn);
		PathNodeMap {
			nodes: nodes,
			key_moves: Vec::<KeyMove2>::with_capacity(128/(std::mem::size_of::<KeyMove2>())),
		}
	}
	pub fn complete_solve_2(&self, base_level: &Level, maps_out: &mut Vec::<PathMap>) {		
		let initial_pn = PathNode {
			pt: Vector(self.level.cmp_data.deer_x as i32, self.level.cmp_data.deer_y as i32),
			move_taken: None,
			prev_node_idx: 0,
		};
		let mut nodes = Vec::<PathNode>::with_capacity(256/(std::mem::size_of::<PathNode>()));
		nodes.push(initial_pn);

		let mut tail_nodes = StackStack16::new(); 
		let mut new_tail_nodes = StackStack16::new(); 	// somewhere to store new tail nodes
		tail_nodes.push(0);
		while tail_nodes.len() != 0 {					// check if map is complete
			for idx in 0..tail_nodes.len() {							// for each tail node
				let tnidx = tail_nodes.stack[idx]; 
				let tnode = nodes[tnidx as usize];
				let pt = tnode.pt;									
				'loop_moves: for movedir in ALLMOVES.iter() {			// for each possible move
					let npt = pt.add_dir(&movedir);						// what is in this direction? let's find out
					if !base_level.vector_in_bounds(&npt) { continue; }
					if base_level.get_obj_at_pt(&npt) != Obj::Wall {
						// first check this point isn't already in our list!!!						
						for n in &nodes {
							if n.pt == npt { continue 'loop_moves; }		// This is a hot spot 9.88%
						}

						// yep, we can move here, make a new tail node
						let pn = PathNode {
							pt: npt.clone(),
							move_taken: Some(movedir.as_move2()),
							prev_node_idx: tnidx as u16,
						};
						new_tail_nodes.push(nodes.len() as u16);
						nodes.push(pn);
					}
				}	
			}
	
			// move new_tail_nodes to tail_nodes
			tail_nodes.clone_from(&new_tail_nodes);
			new_tail_nodes.clear();
		}
		// pnm -> new_by_applying_key_push(pnm, pm, km)
	}

	pub fn apply_key_push_2(&self, nodes: &Vec::<PathNode>, km: &KeyMove2) -> PathMap { 	// after we complete a map, we need to take a key move and start again	
		let mut map_b = self.clone();
				
		// new deer point
		let np = nodes[km.pni as usize].pt.add_dir(&km.move_dir.as_move());

		map_b.level.set_deer_pos(&np);				// move deer
		
		backtrace_moves2(nodes, km.pni as usize, &mut map_b.path);
		map_b.path.push(&km.move_dir);
		
		map_b
	}
	pub fn new_by_applying_key_push(pnm: &PathNodeMap, pm: &PathMap, km: &KeyMove2) -> PathMap { 	// after we complete a map, we need to take a key move and start again	
		let mut map_b = pm.clone();
				
		// new deer point
		let np = pnm.nodes[km.pni as usize].pt.add_dir(&km.move_dir.as_move());

		map_b.level.set_deer_pos(&np);				// move deer
		
		pnm.backtrace_moves(km.pni as usize, &mut map_b.path);
		map_b.path.push(&km.move_dir);
		
		map_b
	}
	pub fn complete_map_solve(&self, base_level: &Level) -> PathNodeMap {
		let mut pnm = self.to_pnm();					// we want complete_map to clone from self
		let mut tail_nodes = StackStack16::new(); 
		let mut new_tail_nodes = StackStack16::new(); 	// somewhere to store new tail nodes
		tail_nodes.push(0);
		while tail_nodes.len() != 0 {					// check if map is complete
			for idx in 0..tail_nodes.len() {							// for each tail node
				let tnidx = tail_nodes.stack[idx]; 
				let tnode = pnm.nodes[tnidx as usize];
				let pt = tnode.pt;									
				'loop_moves: for movedir in ALLMOVES.iter() {			// for each possible move
					let npt = pt.add_dir(&movedir);						// what is in this direction? let's find out
					if !base_level.vector_in_bounds(&npt) { continue; }
					if base_level.get_obj_at_pt(&npt) != Obj::Wall {
						// first check this point isn't already in our list!!!						
						for n in &pnm.nodes {
							if n.pt == npt { continue 'loop_moves; }		// This is a hot spot 9.88%
						}

						// yep, we can move here, make a new tail node
						let pn = PathNode {
							pt: npt.clone(),
							move_taken: Some(movedir.as_move2()),
							prev_node_idx: tnidx as u16,
						};
						new_tail_nodes.push(pnm.nodes.len() as u16);
						pnm.nodes.push(pn);
					}
				}	
			}
	
			// move new_tail_nodes to tail_nodes
			tail_nodes.clone_from(&new_tail_nodes);
			new_tail_nodes.clear();
		}
		pnm
	}

}

impl PathNodeMap {
	pub fn apply_key_pushes(&self, base_path_map: &PathMap) -> Vec<PathMap> {			// avoid using these as they are slow
		let mut nmaps = Vec::<PathMap>::with_capacity(self.key_moves.len());
		for km in &self.key_moves {	
			nmaps.push(PathMap::new_by_applying_key_push(self, base_path_map, &km));
		}
		nmaps
	}
	pub fn backtrace_moves(&self, pni: usize, spath: &mut ShrunkPath) {		// 5.5, 2.9
		let mut path = StackStack8::new();
		// start at pn and work backwards
		let mut pnr = &self.nodes[pni];
		loop {
			if pnr.move_taken.is_some() {
				path.push(pnr.move_taken.unwrap() as u8);
				if pnr.prev_node_idx == 0 {
					let m = &self.nodes[0].move_taken;
					if m.is_some() { path.push(m.unwrap() as u8); }
					break;
				}
				pnr = &self.nodes[pnr.prev_node_idx as usize];
			} else {
				break;
			}
		}
		
		for i in 1..=path.next {
			let rev = path.next - i;
			spath.push_u8(path.stack[rev]);	//3.88%
		}
	}
}


pub fn backtrace_moves2(nodes: &Vec::<PathNode>, pni: usize, spath: &mut ShrunkPath) {		// 5.5, 2.9
	let mut path = StackStack8::new();
	// start at pn and work backwards
	let mut pnr = nodes[pni];
	loop {
		if pnr.move_taken.is_some() {
			path.push(pnr.move_taken.unwrap() as u8);
			if pnr.prev_node_idx == 0 {
				let m = nodes[0].move_taken;
				if m.is_some() { path.push(m.unwrap() as u8); }
				break;
			}
			pnr = nodes[pnr.prev_node_idx as usize];
		} else {
			break;
		}
	}
	
	for i in 1..=path.next {
		let rev = path.next - i;
		spath.push_u8(path.stack[rev]);	//3.88%
	}
}

