use itertools::Itertools;
//use std::collections::{*};



fn str_truncate(s: &str, max_width: usize) -> String {
    s.chars().take(max_width).collect()
}

fn str_append_chars(s: &str, chars: &[char]) -> String {
	let mut s2 = String::from(s);
	for c in chars {
		s2.push(*c);
	}
	s2
}



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
	pub id: String,
	pub value: Option<bool>,
	pub output_gates: Vec<usize>,
	pub name: String,
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
	input_a_id: String,
	input_b_id: String,
	output_id: String,
	op: Operation,
	name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Circuit {
	pub wires: Vec<Wire>,
	pub gates: Vec<Gate>,
}

impl Circuit {
	fn new() -> Self {
		Self {
			wires: Vec::<Wire>::new(),
			gates: Vec::<Gate>::new(),
		}
	}	
	fn get_gate_idx_by_name(&self, name: &str) -> Vec<usize> {
		self.gates.iter().enumerate().filter(|(_i,g)| str_truncate(&g.name, name.len()) == name).map(|(i,_g)| i).collect_vec()
	}
	fn get_mut_wire_by_id(&mut self, id: &str) -> Option<&mut Wire> {
		self.wires.iter_mut().find(|a| a.id == id)
	}
	fn get_wire_by_id(&self, id: &str) -> Option<&Wire> {
		self.wires.iter().find(|a| a.id == id)
	}
	fn get_wire_idx_by_id(&self, id: &str) -> Option<usize> {
		self.wires.iter().position(|a| a.id == id)
	}
	fn run_calculation(&mut self) {
		let mut gates_processed = 1;
		while gates_processed > 0 {
			gates_processed = 0;
			for gate_idx in 0..self.gates.len() {
				if self.gates[gate_idx].output_value.is_none() {
					let a = self.get_wire_by_id(&self.gates[gate_idx].input_a_id).unwrap().value;
					let b = self.get_wire_by_id(&self.gates[gate_idx].input_b_id).unwrap().value;
					if a.is_some() && b.is_some() {
						let output_value = Some(self.gates[gate_idx].op.operate(a.unwrap(), b.unwrap()));
						self.gates[gate_idx].output_value = output_value;
						let wire_id = self.gates[gate_idx].output_id.clone();
						self.get_mut_wire_by_id(&wire_id).unwrap().update(output_value.unwrap());
						gates_processed += 1;
					}
				}
			}
			// println!("gates processed: {}", gates_processed);
		}
	}
	fn set_input(&mut self, input_x: u64, input_y: u64) {
		// check input is in range
		assert!(input_x <= (0x01 << 44));
		assert!(input_y <= (0x01 << 44));
		// get all the input wires
		let mut x_wires = self.wires.iter().enumerate().filter(|(_i,&ref k)| k.id.chars().nth(0)==Some('x')).map(|(i,&ref k)| (k.id.clone(), i)).collect_vec();
		let mut y_wires = self.wires.iter().enumerate().filter(|(_i,&ref k)| k.id.chars().nth(0)==Some('y')).map(|(i,&ref k)| (k.id.clone(), i)).collect_vec();
		// sort them x00..x44 and y00..y44
		x_wires.sort_by(|a,b| a.0.cmp(&b.0));
		y_wires.sort_by(|a,b| a.0.cmp(&b.0));
		// set values
		for (i,(_id, idx)) in x_wires.iter().enumerate() {
			self.wires[*idx].value = Some(((input_x >> i) & 0x01) != 0);
		}
		for (i,(_id, idx)) in y_wires.iter().enumerate() {
			self.wires[*idx].value = Some(((input_y >> i) & 0x01) != 0);
		}		
	}
	fn test_row(&mut self, row: usize) -> bool {
		// true for valid, false for invalid
		// test 0,0 0,1 1,0 1,1 and look for valid outputs
		// we will also test overflow (carry)		
		assert!(row <= 44);
		let n = 1 << row;

		self.set_input(0,0);
		println!("0x{:012x}, 0x{:012x}", self.get_input_x(), self.get_input_y());
		self.run_calculation();
		let r = self.get_output();
		println!("  output: {:012x}  ", r);
		if (r>>row) & 0x03 != 0 {
			return false;				
		}
		
		self.set_input(n,0);
		println!("0x{:012x}, 0x{:012x}", self.get_input_x(), self.get_input_y());
		self.run_calculation();
		let r = self.get_output();
		println!("  output: {:012x}  ", r);
		if (r>>row) & 0x03 != 1 {
			return false;
		}
		
		self.set_input(0,n);
		println!("0x{:012x}, 0x{:012x}", self.get_input_x(), self.get_input_y());
		self.run_calculation();
		let r = self.get_output();
		println!("  output: {:012x}  ", r);
		if (r>>row) & 0x03 != 1 {
			return false;
		}
		
		// this last one tests for a positive carry 
		self.set_input(n,n);
		println!("0x{:06x}, 0x{:012x}", self.get_input_x(), self.get_input_y());
		self.run_calculation();
		let r = self.get_output();
		println!("  output: {:012x}  ", r);
		if (r>>row) & 0x03 != 2 {
			return false;
		}
		
		return true;
	}
	fn get_output(&self) -> u64 {
		// get all the zed wires
		let mut outputs = self.wires.iter().filter(|&ref k| k.id.chars().nth(0)==Some('z')).map(|&ref k| (k.id.clone(), k.value)).collect_vec();
		// sort them
		outputs.sort_by(|a,b| a.0.cmp(&b.0));
		// find solution
		outputs.iter().enumerate().map(|(i, k)| ((k.1.unwrap() as u64) << i)).sum()
	}
	fn get_input_x(&self) -> u64 {	// for testing
		// get all the x wires
		let mut outputs = self.wires.iter().filter(|&ref k| k.id.chars().nth(0)==Some('x')).map(|&ref k| (k.id.clone(), k.value)).collect_vec();
		// sort them
		outputs.sort_by(|a,b| a.0.cmp(&b.0));
		// find solution
		outputs.iter().enumerate().map(|(i, k)| ((k.1.unwrap() as u64) << i)).sum()
	}
	fn get_input_y(&self) -> u64 {	// for testing
		// get all the y wires
		let mut outputs = self.wires.iter().filter(|&ref k| k.id.chars().nth(0)==Some('y')).map(|&ref k| (k.id.clone(), k.value)).collect_vec();
		// sort them
		outputs.sort_by(|a,b| a.0.cmp(&b.0));
		// find solution
		outputs.iter().enumerate().map(|(i, k)| ((k.1.unwrap() as u64) << i)).sum()
	}
}

pub fn day24(input: &String) -> (String, String) {
	// wires and gates
	let lines = input.lines().collect_vec();

	//let mut wires: BTreeMap<String, Wire> = BTreeMap::new();
	//let mut gates: BTreeMap<u16, Gate> = BTreeMap::new();
	let mut circuit = Circuit::new();

	// load in initial wires
	let mut row = 0;
	while !lines[row].is_empty() {
		let wire_id = lines[row][0..=2].to_string();
		let wire_value = lines[row][5..=5].to_string().parse::<u8>().unwrap() != 0;
		circuit.wires.push( Wire { id: wire_id, value: Some(wire_value), output_gates: vec![], name: "".to_string() });
		row += 1;
	}

	// load in gates
	row += 1;
	let mut gate_idx: usize = 0;
	let re = regex::Regex::new(r"([a-z0-9]+) (XOR|AND|OR) ([a-z0-9]+) -> ([a-z0-9]+)").expect("valid regex");
	while row < lines.len() && !lines[row].is_empty() {
		let row_data = re.captures(lines[row]).unwrap().iter().map(|m| m.unwrap().as_str()).collect_vec();
		let input_a_id = row_data[1];
		let op = row_data[2];
		let input_b_id = row_data[3];
		let output_id = row_data[4];

		// initialise associated wires
		if let Some(w) = circuit.get_mut_wire_by_id(input_a_id) {
			w.output_gates.push(gate_idx);
		} else {
			circuit.wires.push( Wire { id: input_a_id.to_string(), value: None, output_gates: vec![gate_idx], name: "".to_string() } );
		}
		if let Some(w) = circuit.get_mut_wire_by_id(input_b_id) {
			w.output_gates.push(gate_idx);
		} else {
			circuit.wires.push( Wire { id: input_b_id.to_string(), value: None, output_gates: vec![gate_idx], name: "".to_string() } );
		}
		if let Some(_w) = circuit.get_mut_wire_by_id(output_id) {
			// do nothing
		} else {
			circuit.wires.push( Wire { id: output_id.to_string(), value: None, output_gates: vec![], name: "".to_string() } );
		}

		circuit.gates.push( Gate {
			output_value: None,
			input_a_id: input_a_id.to_string(),
			input_b_id: input_b_id.to_string(),
			output_id: output_id.to_string(),
			op: Operation::from_str(&op),
			name: "".to_string(),
		});


		gate_idx += 1;
		row += 1;
	}

	// perform calculation
	circuit.run_calculation();

	// get the solution
	// expected 55544677167336 for input24.txt
	let p1soln = circuit.get_output();
	println!("part 1 solution: {}", p1soln);
	
	// part 2
	
	// we will start by identifying which wires / gates are correct, in order to reduce the search space

	println!("\ngate types: ");
	let gate_types = circuit.gates.iter().map(|g| g.op.to_string()).collect_vec();
	let gate_counts = gate_types.iter().map(|k| (k,1)).into_group_map();
	for (k,v) in &gate_counts {
		println!("{}: {}", k, v.iter().sum::<i32>());
	}

	// input wires x00..=x44 and y00..=y44 are valid by default
	let wire_ids = circuit.wires.iter().map(|a| a.id.chars().map(|c| c).collect_vec()).collect_vec();
	let non_input_wires = wire_ids.iter().enumerate().filter(|(_i,wid)| wid[0] != 'x' && wid[0] != 'y').collect_vec();
	println!("input wires: {}", circuit.wires.len() - non_input_wires.len());
	println!("non-input wires: {}", non_input_wires.len());

	// For a FULL-ADDER circuit (i.e. with inputs X01 to X44)
	// There are 3 non-output wires, and 2 output wires (Zn and Cn)
	// For the non-output wires:
	// -- Dn has XOR1 as input, and 2 outputs: XOR2 and AND1.
	// -- En has AND1 as input, and OR as output
	// -- Fn has AND2 as input, and OR as output
	// -- XOR2 outputs to Zn and nothing else
	// -- COUTn outputs to row n+1 XOR2 and row n+1 AND1 (except for row 44, which outputs directly as Z45)
	
	// For a HALF-ADDER circuit (i.e. with X00 as input)
	// There are only 2 outputs: 
	// - Z00, from XOR1
	// - COUT00 from AND1

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

	let mut valid_wire_ids: Vec<String> = Vec::new();	// wires that have their input (source) validated

	// input wires are all valid
	for (_i,w) in circuit.wires.iter().enumerate() {
		let c = w.id.chars().collect_vec()[0];
		if c == 'x' || c == 'y' {
			valid_wire_ids.push(w.id);
		}
	}


	// XOR1 gates are identifiable from input wires
	let mut xor1s = circuit.gates.iter_mut().filter(|g| g.op == Operation::XOR && valid_wire_ids.contains(&g.input_a_id) && valid_wire_ids.contains(&g.input_b_id)).collect_vec();
	xor1s.iter_mut().for_each(|g| {
		let chars = &g.input_a_id.chars().collect_vec()[1..=2];
		g.name = str_append_chars("XOR1_", chars);
	});
	let xor1s = circuit.get_gate_idx_by_name("XOR1_");;

	// AND1 gates are identifiable from input wires
	let mut and1s = circuit.gates.iter_mut().filter(|g| g.op == Operation::AND && valid_wire_ids.contains(&g.input_a_id) && valid_wire_ids.contains(&g.input_b_id)).collect_vec();
	and1s.iter_mut().for_each(|g| {
		let chars = &g.input_a_id.chars().collect_vec()[1..=2];
		g.name = str_append_chars("AND1_", chars);
	});
	let and1s = circuit.get_gate_idx_by_name("AND1_");;

	// AND1 gate 00 should only output to C00 ( AND2_01 and XOR2_01 )
	// AND1 gates 01 to 44 should only output to an OR gate
	// AND2 gates 01 to 44 should only outupt to the same OR Gate
	// AND2_00 does not exist
	
	// Find the AND1 gates and check the output wire to make sure it only outputs to 1 OR gate
	for idx in and1s {
		print!("gate {}: ", circuit.gates[idx].name);
		let op_id = circuit.gates[idx].output_id;
		let op_w = circuit.get_wire_by_id(op_id);
		if op_w.output_gates.len() > 1 {
			print!("(outputs to {} gates) ", op.w.output_gates.len());
		}
		if op_w.


	// check that sxor1_00 outputs directly to z0.
	// let g = gates.get(valid_gates.iter().find("sxor1_00").unwrap()).unwrap();
	// if g.output != "z00" {
		// println!("sxor1_00 has incorrect output wire");
	// }
	// let _ = g;

	// look at the OR gates
	// inputs: cand1_n output, cand2_n output
	// output: wire cn
	// if the or gate has 1 wire from cand1_n as input, we can label it
	// let cand1_wires = cand1.iter().map(|k| gates.get(k).unwrap().output).collect_vec();
	// let mut g = gates.iter_mut()
		// .filter(|(k,v)| v.op == Operation::OR && (cand1_wires.contains(&v.input_a) ^ cand1_wires.contains(&v.input_b))).collect_vec();
	//g.iter_mut().for_each(|(k,v)| v.tags.push("cor_".to_string() + v.input_))

	// cand1_00 should output directly to c00...
	// cand1_01 to cand1_44 will have an OR cor,
	// an AND cand2 (takes in cn_ and sxor1_n)

	// sxor2 is the second xor gate in the sum calculation
	// it takes sxor1_n and c_n-1 as input, and outputs to zn
	// for sxor1_1 to sxor1_44


	(p1soln.to_string(), "no result".to_string())

}