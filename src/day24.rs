use std::cell::RefCell;
use std::rc::Rc;
use itertools::Itertools;

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

// Wire Role is:
// X, Y (inputs)
// Z (output),
// C (carry),
// D, E, F (intermediate values),
// U (unknown)

#[derive(Clone, Debug, Eq, PartialEq)]
struct Wire {
	pub id: String,
	pub value: Option<bool>,
	pub output_gates: Vec<Rc<RefCell<Gate>>>,
	pub role: char,
	pub n: Option<usize>,
}

// Gate Role is the role of the gate in the adder circuit
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum GateRole {
	UNK,
	XOR1,
	XOR2,
	AND1,
	AND2,
	OR,
	XOR,			// for n == 0 only
	AND,			// for n == 0 only
}

impl GateRole {
	pub fn to_string(&self) -> String {
		match self {
			GateRole::UNK => "UNK".to_string(),
			GateRole::XOR1 => "XOR1".to_string(),
			GateRole::XOR2 => "XOR2".to_string(),
			GateRole::AND1 => "AND1".to_string(),
			GateRole::AND2 => "AND2".to_string(),
			GateRole::OR => "OR".to_string(),
			GateRole::XOR => "XOR".to_string(),
			GateRole::AND => "AND".to_string(),
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Gate {
	idx: usize,
	output_value: Option<bool>,
	//input_ids: [ String; 2 ],
	output_id: String,
	op: Operation,
	role: GateRole,
	n: Option<usize>,
}

impl Gate {
	pub fn to_string(&self) -> String {
		let mut s = String::new();
		if self.role == GateRole::UNK {
			s.push_str(&format!("gate UNK (op={}, n={:?})", self.op.to_string(), self.n));
		} else {
			s.push_str(&format!("gate {} (n={:?})", self.role.to_string(), self.n));
		}
		s
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Circuit {
	pub wires: Vec<Rc<RefCell<Wire>>>,
	pub gates: Vec<Rc<RefCell<Gate>>>,
}

impl Circuit {
	fn new() -> Self {
		Self {
			wires: vec![],
			gates: vec![],
		}
	}
	fn get_wire_by_id(&self, id: &str) -> Option<Rc<RefCell<Wire>>> {
		if let Some(r) = self.wires.iter().find(|a| a.borrow().id == id) {
			return Some(r.clone());
		} else {
			return None;
		}
	}
	fn get_gate_input_wires(&self, gate_idx: usize) -> Vec<Rc<RefCell<Wire>>> {
		self.wires.iter().filter(|w| w.borrow().output_gates.iter().any(|g| g.borrow().idx == gate_idx)).cloned().collect_vec()
	}
	fn get_gate_input_ids(&self, gate_idx: usize) -> Vec<String> {
		self.wires.iter().filter(|w| w.borrow().output_gates.iter().any(|g| g.borrow().idx == gate_idx)).cloned().map(|w| w.borrow().id.clone()).collect_vec()
	}
	fn run_calculation(&mut self) {
		let mut gates_processed = 1;
		while gates_processed > 0 {
			gates_processed = 0;
			for gate in self.gates.iter() {
				if gate.borrow().output_value.is_none() {
					let input_wires = self.get_gate_input_wires(gate.borrow().idx);
					let a = input_wires[0].borrow().value.clone();
					let b = input_wires[1].borrow().value;
					if a.is_some() && b.is_some() {
						let output_value = Some(gate.borrow().op.operate(a.unwrap(), b.unwrap()));
						gate.borrow_mut().output_value = output_value;
						let wire_id = gate.borrow().output_id.clone();
						let output_wire = self.get_wire_by_id(&wire_id).unwrap();
						output_wire.borrow_mut().value = output_value;
						gates_processed += 1;
					}
				}
			}
			// println!("gates processed: {}", gates_processed);
		}
	}
	fn _set_input(&mut self, input_x: u64, input_y: u64) {
		// check input is in range
		assert!(input_x <= (1 << 44));
		assert!(input_y <= (1 << 44));
		// get all the input wires
		let mut x_wires = self.wires.iter().enumerate().filter(|(_i,&ref k)| k.borrow().id.chars().nth(0)==Some('x')).map(|(i,&ref k)| (k.borrow().id.clone(), i)).collect_vec();
		let mut y_wires = self.wires.iter().enumerate().filter(|(_i,&ref k)| k.borrow().id.chars().nth(0)==Some('y')).map(|(i,&ref k)| (k.borrow().id.clone(), i)).collect_vec();
		// sort them x00..x44 and y00..y44
		x_wires.sort_by(|a,b| a.0.cmp(&b.0));
		y_wires.sort_by(|a,b| a.0.cmp(&b.0));
		// set values
		for (i,(_id, idx)) in x_wires.iter().enumerate() {
			self.wires[*idx].borrow_mut().value = Some(((input_x >> i) & 0x01) != 0);
		}
		for (i,(_id, idx)) in y_wires.iter().enumerate() {
			self.wires[*idx].borrow_mut().value = Some(((input_y >> i) & 0x01) != 0);
		}		
	}
	fn get_output(&self) -> u64 {
		// get all the zed wires
		let mut outputs = self.wires.iter().filter(|&ref k| k.borrow().id.chars().nth(0)==Some('z')).map(|&ref k| (k.borrow().id.clone(), k.borrow().value)).collect_vec();
		// sort them
		outputs.sort_by(|a,b| a.0.cmp(&b.0));
		// find solution
		outputs.iter().enumerate().map(|(i, k)| ((k.1.unwrap() as u64) << i)).sum()
	}
	fn _get_input_x(&self) -> u64 {	// for testing
		// get all the x wires
		let mut outputs = self.wires.iter().filter(|&ref k| k.borrow().id.chars().nth(0)==Some('x')).map(|&ref k| (k.borrow().id.clone(), k.borrow().value)).collect_vec();
		// sort them
		outputs.sort_by(|a,b| a.0.cmp(&b.0));
		// find solution
		outputs.iter().enumerate().map(|(i, k)| ((k.1.unwrap() as u64) << i)).sum()
	}
	fn _get_input_y(&self) -> u64 {	// for testing
		// get all the y wires
		let mut outputs = self.wires.iter().filter(|&ref k| k.borrow().id.chars().nth(0)==Some('y')).map(|&ref k| (k.borrow().id.clone(), k.borrow().value)).collect_vec();
		// sort them
		outputs.sort_by(|a,b| a.0.cmp(&b.0));
		// find solution
		outputs.iter().enumerate().map(|(i, k)| ((k.1.unwrap() as u64) << i)).sum()
	}
	fn _very_basic_test(&mut self, expected: u64) -> bool {
		// self.set_input(a,b);
		//self.reset();
		self.run_calculation();
		self.get_output() == expected
	}
	fn _swap_gate_outputs(&self, a: Rc<RefCell<Gate>>, b: Rc<RefCell<Gate>>) {
		let av = a.borrow().output_id.clone();
		let bv = b.borrow().output_id.clone();
		b.borrow_mut().output_id = av;
		a.borrow_mut().output_id = bv;
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
		circuit.wires.push( Rc::from(RefCell::from( Wire {
			id: wire_id,
			value: Some(wire_value),
			output_gates: vec![],
			role: 'U',
			n: None,
		})));
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

		// create the gate
		let gate = Rc::from(RefCell::from( Gate {
			idx: gate_idx,
			output_value: None,
			//input_ids: [ input_a_id.to_string(), input_b_id.to_string() ],
			output_id: output_id.to_string(),
			op: Operation::from_str(&op),
			role: GateRole::UNK,
			n: None,
		}));

		// initialise associated wires
		for input_id in [ input_a_id, input_b_id ].into_iter() {
			if let Some(w) = circuit.get_wire_by_id(input_id) {
				w.borrow_mut().output_gates.push(gate.clone());
			} else {
				circuit.wires.push(Rc::from(RefCell::from( Wire {
					id: input_id.to_string(),
					value: None,
					output_gates: vec![gate.clone()],
					role: 'U',
					n: None,
				})));
			}
		}

		if let None = circuit.get_wire_by_id(output_id) {
			circuit.wires.push( Rc::from(RefCell::from( Wire {
				id: output_id.to_string(),
				value: None,
				output_gates: vec![],
				role: 'U',
				n: None,
			} )));
		}

		circuit.gates.push(gate);
		row += 1;
		gate_idx += 1;
	}

	// perform calculation
	circuit.run_calculation();

	// get the solution
	// expected 55544677167336 for input24.txt
	let p1soln = circuit.get_output();
	println!("part 1 solution: {}", p1soln);
	
	// part 2
	// see notes24.txt for digital logic
	
	// analyse gate types and wire types

	println!("\ngate types: ");
	let gate_types = circuit.gates.iter().map(|g| g.borrow().op.to_string()).collect_vec();
	let gate_counts = gate_types.iter().map(|k| (k,1)).into_group_map();
	for (k,v) in &gate_counts {
		println!("{}: {}", k, v.iter().sum::<i32>());
	}

	let wire_ids = circuit.wires.iter().map(|a| a.borrow().id.chars().map(|c| c).collect_vec()).collect_vec();
	let non_input_wires = wire_ids.iter().enumerate().filter(|(_i,wid)| wid[0] != 'x' && wid[0] != 'y').collect_vec();
	println!("input wires: {}", circuit.wires.len() - non_input_wires.len());
	println!("non-input wires: {}", non_input_wires.len());

	// input wires are all valid

	let mut valid_wire_ids: Vec<String> = Vec::new();

	for w in circuit.wires.iter() {
		let c = w.borrow().id.chars().collect_vec()[0];
		if c == 'x' || c == 'y' {
			valid_wire_ids.push(w.borrow().id.clone());
			w.borrow_mut().role = if c == 'x' {
				'X'
			} else {
				'Y'
			};
			let n = w.borrow().id[1..=2].parse::<usize>().unwrap();
			w.borrow_mut().n = Some(n);
		}
	}

	// XOR1 gates are identifiable from input wires

	let xor1s = circuit.gates.iter()
		.filter(|g| g.borrow().op == Operation::XOR &&
			circuit.get_gate_input_ids(g.borrow().idx).iter().all(|wid| valid_wire_ids.iter().any(|id| id==wid)))
		.map(|a| a.clone()).collect_vec();

	xor1s.iter().for_each(|g| {
		let input_ids = circuit.get_gate_input_ids(g.borrow().idx);
		let n = input_ids[0][1..=2].parse::<usize>().unwrap();
		let n2 = input_ids[1][1..=2].parse::<usize>().unwrap();
		if n != n2 {
			println!("ERROR: input ids do not match");
		}
		g.borrow_mut().role = if n == 0 {
			GateRole::XOR
		} else {
			GateRole::XOR1
		};
		g.borrow_mut().n = Some(n);
	});

	if xor1s.len() != 45 {
		println!("ERROR: Invalid number of XOR1 gates. Expected 45, got {}", xor1s.len());
	} else {
		// we can label the rest of the XOR gates
		circuit.gates.iter().filter(|g| g.borrow().role == GateRole::UNK && g.borrow().op == Operation::XOR).for_each(|g| {
			g.borrow_mut().role = GateRole::XOR2;
		});
	}

	// AND1 gates are identifiable from input wires

	let and1s = circuit.gates.iter()
		.filter(|g| g.borrow().op == Operation::AND &&
			circuit.get_gate_input_ids(g.borrow().idx).iter().all(|wid| valid_wire_ids.iter().any(|id| id==wid)))
		.map(|a| a.clone()).collect_vec();

	and1s.iter().for_each(|g| {
		let input_ids = circuit.get_gate_input_ids(g.borrow().idx);
		let n = input_ids[0][1..=2].parse::<usize>().unwrap();
		let n2 = input_ids[1][1..=2].parse::<usize>().unwrap();
		if n != n2 {
			println!("ERROR: input ids do not match");
		}
		g.borrow_mut().role = if n == 0 {
			GateRole::AND
		} else {
			GateRole::AND1
		};
		g.borrow_mut().n = Some(n);
	});

	if and1s.len() != 45 {
		println!("ERROR: Invalid number of AND1 gates. Expected 45, got {}", xor1s.len());
	} else {
		// we can label the rest of the AND gates
		circuit.gates.iter().filter(|g| g.borrow().role == GateRole::UNK && g.borrow().op == Operation::AND).for_each(|g| {
			g.borrow_mut().role = GateRole::AND2;
		});
	}

	// All OR gate have role GateRole::OR

	circuit.gates.iter().filter(|g| g.borrow().op == Operation::OR).for_each(|g| {
		g.borrow_mut().role = GateRole::OR;
	});

	// All gates should have a role now

	let unassigned = circuit.gates.iter().filter(|g| g.borrow().role == GateRole::UNK).collect_vec();
	if unassigned.len() > 0 {
		println!("ERROR: Some gates do not have a role!");
		return (p1soln.to_string(), "unknown".to_string());
	}


	// find definitely invalid wires, by looking to see if gate output matches desired kind of gate output
	println!("Finding definitely invalid gate outputs");
	let mut dodgy_gates: Vec<Rc<RefCell<Gate>>> = vec![];

	for g in circuit.gates.iter() {
		let role = g.borrow().role;
		let output_id = g.borrow().output_id.clone();
		let mut is_dodgy = false;
		match role {
			GateRole::XOR => {
				// test that output goes to z00
				if g.borrow().output_id != "z00" {
					is_dodgy = true;
				}
			},
            GateRole::XOR1 => {
				// test that output goes to XOR2 and AND2
				let output_gates = circuit.get_wire_by_id(&output_id).unwrap().borrow().output_gates.clone();
				let output_gate_roles = output_gates.iter().map(|og| og.borrow().role.clone()).collect_vec();
				if output_gate_roles != vec![GateRole::XOR2, GateRole::AND2] && output_gate_roles != vec![GateRole::AND2, GateRole::XOR2] {
					is_dodgy = true;
				}
			},
			GateRole::XOR2 => {
				// test XOR2 outputs to Z
				if &g.borrow().output_id[0..1] != "z" {
					is_dodgy = true;
				}
			},
			GateRole::AND => {
				// check outputs to XOR2 and AND2
				let output_gates = circuit.get_wire_by_id(&output_id).unwrap().borrow().output_gates.clone();
				let output_gate_roles = output_gates.iter().map(|og| og.borrow().role.clone()).collect_vec();
				if output_gate_roles != vec![GateRole::XOR2, GateRole::AND2] && output_gate_roles != vec![GateRole::AND2, GateRole::XOR2] {
					is_dodgy = true;
				}
			},
			GateRole::AND1 => {
				// check it outputs to OR gate
				let output_gates = circuit.get_wire_by_id(&output_id).unwrap().borrow().output_gates.clone();
				let output_gate_roles = output_gates.iter().map(|og| og.borrow().role.clone()).collect_vec();
				if output_gate_roles != vec![GateRole::OR] {
					is_dodgy = true;
				}
			},
			GateRole::AND2 => {
				// output must go to an OR gate
				let output_gates = circuit.get_wire_by_id(&output_id).unwrap().borrow().output_gates.clone();
				let output_gate_roles = output_gates.iter().map(|og| og.borrow().role.clone()).collect_vec();
				if output_gate_roles != vec![GateRole::OR] {
					is_dodgy = true;
				}
			},
			GateRole::OR => {
				// output must be either z45 or go to XOR2 and AND2
				if output_id != "z45" {
					let output_gates = circuit.get_wire_by_id(&output_id).unwrap().borrow().output_gates.clone();
					let output_gate_roles = output_gates.iter().map(|og| og.borrow().role.clone()).collect_vec();
					if output_gate_roles != vec![GateRole::XOR2, GateRole::AND2] && output_gate_roles != vec![GateRole::AND2, GateRole::XOR2] {
						is_dodgy = true;
					}
				}
			},
			GateRole::UNK => { panic!("shouldn't be able to reach here"); },
		}
		if is_dodgy {
			dodgy_gates.push(g.clone());
			println!("dodgy! {} at idx {} with output_id {}", g.borrow().to_string(), g.borrow().idx, output_id);
		}
	}

	println!("--- total {} dodgy gate outputs found ---", dodgy_gates.len());

	let mut dodgy_ids = dodgy_gates.iter().map(|g| g.borrow().output_id.clone()).collect_vec();
	dodgy_ids.sort();
	let p2result = itertools::Itertools::intersperse(dodgy_ids.into_iter(), ",".to_string()).collect_vec();
	let p2result_s: String = p2result.into_iter().collect();
	println!("part 2 result: {}", p2result_s);

	(p1soln.to_string(), p2result_s)
}