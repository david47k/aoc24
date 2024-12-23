use itertools::Itertools;
use std::collections::{*};

#[derive(Debug,Clone)]
struct Node {
	conns: BTreeSet<[u8;2]>,
}

pub fn day23(input: &String) -> (String, String) {
	let pairs: Vec<([u8;2],[u8;2])> = input.lines().filter(|s| s.len() > 0).map(|s| s.as_bytes()).map(|s| ([s[0], s[1]], [s[3], s[4]])).collect_vec();
	let mut nodes: BTreeMap<[u8;2],Node> = BTreeMap::new();

	// create node graph
	for (a,b) in pairs.iter() {
		if let Some(&mut ref mut x) = nodes.get_mut(a) {
			x.conns.insert([b[0], b[1]]);
		} else {
			nodes.insert(*a, Node { conns: BTreeSet::from( [[b[0], b[1]]] ) } );
		}
		if let Some(&mut ref mut x) = nodes.get_mut(b) {
			x.conns.insert([a[0], a[1]]);
		} else {
			nodes.insert(*b, Node { conns: BTreeSet::from( [[a[0], a[1]]] ) } );
		}
	}

	println!("day 23 part 1");
	// println!("nodes len: {}", nodes.len());
	// for (k,v) in &nodes {
	// 	println!("{}: {}", nid2s(k), v.conns.len())
	// }

	fn nid2s(nid: &[u8;2]) -> String {
		let mut s = String::new();
		s.push(nid[0] as char);
		s.push(nid[1] as char);
		s
	}

	let mut trios: Vec<Vec<[u8;2]>> = vec![];
	let mut t_trios: Vec<Vec<[u8;2]>> = vec![];
	// find a trio of nodes
	for (id, node) in nodes.iter() {
		// see if it's a trio
		// if it is, then two of our conns will connect to each other!
		if node.conns.len() < 2 {
			continue; // needs to have at least two connections!
		}

		// get all pair combinations of our conns
		let pairs = node.conns.iter().combinations(2)	// we want each pair to have reference to each other
			.filter(|v| nodes.get(v[0]).unwrap().conns.contains(v[1]))
			.map(|v| [v[0], v[1]])
			.collect_vec();

		for [p1,p2] in pairs {
			// we should check that trios doesn't contain a variation of this trio!
			let combos = vec![*id,*p1,*p2].into_iter().permutations(3).collect_vec();
			let ok = combos.iter().all(|c| !trios.contains(c));
			if ok {
				//println!("trio found: {}-{}-{}", nid2s(&id), nid2s(&p1), nid2s(&p2));
				trios.push(vec![*id, *p1, *p2]);
			}
		}
	}

	let nt_trios = trios.iter().filter(|&t| t.iter().any(|n| n[0] == b't')).map(|t| t.clone()).collect_vec();
	t_trios.extend(nt_trios);

	// for t in &t_trios {
	// 	println!("t-trio: {}-{}-{}", nid2s(&t[0]), nid2s(&t[1]), nid2s(&t[2]));
	// }

	let p1result = t_trios.len();
	println!("part 1 result: {}", p1result);

	// part 2

	println!("day 23 part 2");

	// find the largest set
	// every computer in the set is joined to every other computer in the set...
	// conns are bi-directional...
	// there are 520 nodes in total, and each node has connection to 13 other computers
	// so the largest possible set is 14
	// each member of the set will have a large number of their conns
	// the same as each other member of the set

	let mut best_set: BTreeSet<[u8;2]> = BTreeSet::new();

	for (id, node) in &nodes {
		// count how many of our conns, are in our conns's conns
		let mut test_set: BTreeSet<[u8;2]> = BTreeSet::from(node.conns.clone());
		test_set.insert(*id);		// test set of size 14
		let mut reducing_set;

		// how do we determine which subnodes are strong and which are weak ?
		// we need a threshold!

		// print test set
		// print!("test_set: ");
		// for n in &test_set {
		// 	print!("{} ", nid2s(n));
		// }
		// println!("");

		for threshold in (3..=13).rev() {
			reducing_set = test_set.clone();	// test set that will be reduced to what is common between most
			let mut ok_count = 0;
			for subnodeid in test_set.iter() {
				let sub_set: BTreeSet<[u8; 2]> = nodes.get(subnodeid).unwrap().conns.intersection(&reducing_set).map(|&n| n).collect();
				if sub_set.len() >= threshold {
					reducing_set = sub_set;
					reducing_set.insert(*subnodeid);
					ok_count += 1;
					// print matches
					// print!("  subnode {} matches ", nid2s(subnodeid));
					// for n in &reducing_set {
					// 	print!("{} ", nid2s(n));
					// }
					// println!("");
				}
			}
			if ok_count >= threshold {
				if reducing_set.len() > best_set.len() {
					// found best set so far
					// println!("found best set so far, size {}", reducing_set.len());
					best_set = reducing_set.clone();
				}
			}
		}
	}

	println!("best set size {}", best_set.len());
	let s: String = best_set.iter().map(|id| nid2s(id)).intersperse(",".to_string()).collect();
	println!("part 2 result: {}", s);

	(p1result.to_string(), s)
}