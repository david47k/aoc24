use itertools::Itertools;
use std::collections::{*};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Operation {
	AND,
	OR,
	XOR,
}

impl Operation {
	fn from_str(s: &str) -> Operation {
		match s {
			"AND" => Operation::AND,
			"OR" => Operation::OR,
			"XOR" => Operation::XOR,
			_ => panic!(),
		}
	}
	fn to_string(&self) -> String {
		match self {
			Operation::AND => "AND".to_string(),
			Operation::OR => "OR".to_string(),
			Operation::XOR => "XOR".to_string(),
		}
	}
	fn operate(&self, left: bool, right: bool) -> bool {
		match self {
			Operation::AND => left && right,
			Operation::OR => left || right,
			Operation::XOR => left ^ right,
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Wire {
	pub value: Option<bool>,
	//pub input: Option<u16>,
	pub outputs: Vec<u16>,
	pub tags: Vec<String>,
}

impl Wire {
	pub fn update(&mut self, value: bool) {
		if self.value.is_none() {
			self.value = Some(value);
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Gate {
	output_value: Option<bool>,
	input_a: String,
	input_b: String,
	output: String,
	op: Operation,
	tags: Vec<String>,
}

pub fn day24(input: &String) -> (String, String) {
	// wires and gates
	let lines = input.lines().collect_vec();

	let mut wires: BTreeMap<String, Wire> = BTreeMap::new();
	let mut gates: BTreeMap<u16, Gate> = BTreeMap::new();

	// load in initial wires
	let mut row = 0;
	while !lines[row].is_empty() {
		let wire_name = lines[row][0..=2].to_string();
		let wire_value = lines[row][5..=5].to_string().parse::<u8>().unwrap() != 0;
		wires.insert(wire_name, Wire { value: Some(wire_value), outputs: vec![], tags: vec![] });
		row += 1;
	}

	// load in gates
	row += 1;
	let mut gate_id: u16 = 0;
	let re = regex::Regex::new(r"([a-z0-9]+) (XOR|AND|OR) ([a-z0-9]+) -> ([a-z0-9]+)").expect("valid regex");
	while row < lines.len() && !lines[row].is_empty() {
		let mut row_data = re.captures(lines[row]).unwrap().iter().map(|m| m.unwrap().as_str()).collect_vec();
		let input_a_name = row_data[1];
		let op = row_data[2];
		let input_b_name = row_data[3];
		let output_name = row_data[4];

		// initialise associated wires
		if let Some(k) = wires.get_mut(input_a_name) {
			k.outputs.push(gate_id);
		} else {
			wires.insert( input_a_name.to_string(), Wire { value: None,  outputs: vec![gate_id], tags: vec![]} );
		}
		if let Some(k) = wires.get_mut(input_b_name) {
			k.outputs.push(gate_id);
		} else {
			wires.insert( input_b_name.to_string(), Wire { value: None,  outputs: vec![gate_id], tags: vec![] } );
		}
		if let Some(k) = wires.get_mut(output_name) {
			k.input = Some(gate_id);
		} else {
			wires.insert( output_name.to_string(), Wire { value: None, outputs: vec![], tags: vec![] } );
		}

		gates.insert(gate_id, Gate {
			output_value: None,
			input_a: input_a_name.to_string(),
			input_b: input_b_name.to_string(),
			output: output_name.to_string(),
			op: Operation::from_str(&op),
			tags: vec![],
		});


		gate_id += 1;
		row += 1;
	}

	let mut gates_processed = 1;
	while gates_processed > 0 {
		gates_processed = 0;
		for (gate_id, gate) in &mut gates {
			if gate.output_value.is_none() {
				let a = wires.get(&gate.input_a).unwrap().value;
				let b = wires.get(&gate.input_b).unwrap().value;
				if a.is_some() && b.is_some() {
					gate.output_value = Some(gate.op.operate(a.unwrap(), b.unwrap()));
					wires.get_mut(&gate.output).unwrap().update(gate.output_value.unwrap());
					gates_processed += 1;
				}
			}
		}
		// println!("gates processed: {}", gates_processed);
	}

	// get all the zed wires
	let mut outputs = wires.iter().filter(|(&ref k,&ref v)| k.chars().nth(0)==Some('z')).map(|(&ref k,&ref v)| (k.clone(), v.value)).collect_vec();
	outputs.sort();
	let p1soln: u64 = outputs.iter().enumerate().map(|(i, (_, v))| ((v.unwrap() as u64) << i)).sum();

	println!("part 1 solution: {}", p1soln);

	let gate_types = gates.iter().map(|(_,g)| g.op.to_string()).collect_vec();
	let counts = gate_types.iter().map(|k| (k,1)).into_group_map();
	for (k,v) in &counts {
		println!("{}: {}", k, v.iter().sum::<i32>());
	}

	let wire_types = wires.iter().map(|(k,_)| k.chars().map(|c| c).collect_vec()).collect_vec();
	let wire_types = wire_types.into_iter().filter(|wn| wn[0] != 'x' && wn[0] != 'y').collect_vec();
	println!("non-input wires: {}", wire_types.len());


	// part 2

	// four pairs of gate output wires need to be swapped
	// so the machine works as an adder
	// there's about 221 gates
	// so roughly 221 x 220 x 219 x 218 x 217 x 216 x 215 x 214 possibilities
	// output wires are on z00 to z45
	// input wires are on x00 to x44 and y00 to y44
	// half adder has an XOR and AND gate, to make sum + carry
	// full adder has two half adders plus OR gate,
	//     so 2 XOR, 2 AND, and 1 OR (5 gates)
	// XOR: 89: 45 sxor1 gates (x/y00 to x/y44) + 44 sxor2 gates (sxor1_01/c01 to sxor1_44/c44).
	// AND: 89 .. 45 + 44 gates
	// OR: 44
	// non-io wires: 176
	//
	// adder n:
	// zn = (xn XOR yn) XOR cn_
	// cn = (xn AND yn) OR ((xn XOR yn) AND cn_)
	//
	// adder 0:
	// z0 = xn XOR yn
	// c0 = xn AND yn
	//
	// final value:
	// z45 = c44


	let mut valid_wires: Vec<String> = Vec::new();	// wires that have their input (source) validated
	let mut valid_gates: Vec<u16> = Vec::new();		// gates with valid input wires

	// need to identify valid wires, and invalid wires
	// input wires x00..=x44 and y00..=y44 are valid by default

	valid_wires.extend(wires.iter().map(|(k,_)| k.chars().map(|c| c).collect_vec()).filter(|vc| vc[0]=='x' || vc[0] == 'y').collect_vec());

	// it can help to identify gates with valid inputs, too!
	// input sum gates xn XOR yn are valid INPUTS by default, call them sum xor 1. 45 of them.

	let mut sxor1 = gates.iter_mut().filter(|k,v| v.op == Operation::XOR && valid_wires.contains(v.input_a) && valid_wires.contains(v.input_b)).collect_vec();
	sxor1.iter_mut().for_each(|(k,v)| v.tags.push("sxor1_".to_string() + v.input_a.chars()[1..=2]));
	let sxor1 = sxor1.iter().map(|(&k,_)| k).collect_vec();	// only keep the keys, don't keep reference to gates
	valid_gates.extend(sxor1);

	// xn AND yn is the first gate in the carry calculation
	// we'll label them cand1 (carry and 1). 45 of them.
	// the gate inputs will be valid.
	let mut cand1 = gates.iter_mut().filter(|k,v| v.op == Operation::AND && valid_wires.contains(v.input_a) && valid_wires.contains(v.input_b)).collect_vec();
	cand1.iter_mut().for_each(|(k,v)| v.tags.push("cand1_".to_string() + v.input_a.chars()[1..=2]));
	let cand1 = cand1.iter().map(|(&k,_)| k).collect_vec();	// only keep the keys, don't keep reference to gates
	valid_gates.extend(cand1);

	// check that sxor1_00 outputs directly to z0.
	let g = gates.get(valid_gates.iter().find("sxor1_00").unwrap()).unwrap();
	if g.output != "z00" {
		println!("sxor1_00 has incorrect output wire");
	}
	let _ = g;

	// look at the OR gates
	// inputs: cand1_n output, cand2_n output
	// output: wire cn
	// if the or gate has 1 wire from cand1_n as input, we can label it
	let cand1_wires = cand1.iter().map(|k| gates.get(k).unwrap().output).collect_vec();
	let mut g = gates.iter_mut()
		.filter(|(k,v)| v.op == Operation::OR && (cand1_wires.contains(&v.input_a) ^ cand1_wires.contains(&v.input_b))).collect_vec();
	//g.iter_mut().for_each(|(k,v)| v.tags.push("cor_".to_string() + v.input_))

	// cand1_00 should output directly to c00...
	// cand1_01 to cand1_44 will have an OR cor,
	// an AND cand2 (takes in cn_ and sxor1_n)

	// sxor2 is the second xor gate in the sum calculation
	// it takes sxor1_n and c_n-1 as input, and outputs to zn
	// for sxor1_1 to sxor1_44



	fn test_addition() -> bool {
		// return true if addition seems to be working
		// reset all gate output values and wire values
		// set inputs on wires x.. and y..
		// run the machine
		// look for expected output on z...
		false

	}


	("no result".to_string(), "no result".to_string())

}