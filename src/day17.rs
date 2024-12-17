use itertools::Itertools;
//use std::collections::{*};
//use crate::grid::{*};
//use crate::vector::{*};

#[derive(Debug)]
struct Computer {
	pub reg: [u64; 3],		// A, B, C
	pub ip: usize,
	pub program: Vec<u8>,
	pub output: Vec<u64>,
}

const REG_A: usize = 0;
const REG_B: usize = 1;
const REG_C: usize = 2;

impl Computer {
	pub fn new(program: Vec<u8>) -> Computer {
		Self {
			reg: [0; 3],
			ip: 0,
			program,
			output: vec![],
		}
	}
	pub fn step(&mut self) -> bool {		// return true if running, false if halted
		// are we halted?
		if self.ip >= self.program.len() {
			return false;
		}
		let instruction = self.program[self.ip];
		let operand = self.program[self.ip + 1];

		println!("instruction {}, operand {}, at ip {}", instruction, operand, self.ip);
		match instruction {
			0 | 6 | 7 => { // adv: division, operand: combo, 0: output to A, 6: output to B, 7: output to C.
				let n = self.reg[REG_A];
				let d = 2_u32.pow(self.get_combo_value(operand) as u32);
				let r = (n / d as u64);
				let reg_num: usize = match instruction {
					0 => REG_A,
					6 => REG_B,
					7 => REG_C,
					_ => panic!("divide: invalid destination register"),
				};
				self.reg[reg_num] = r;
				self.ip += 2;
				println!("_dv: REG_A:{n} D:{d} R:{r}");
			},
			1 => { // bxl: bitwise xor, operand: literal
				self.reg[REG_B] = self.reg[REG_B] ^ operand as u64;
				self.ip += 2;
			},
			2 => { // bst: mod 8 store to B, operand: combo
				let r = self.get_combo_value(operand) & 0x07;
				self.reg[REG_B] = r;
				self.ip += 2;
			},
			3 => { // jnz: jump. no ip increase. operand: literal
				if self.reg[REG_A] == 0 {
					self.ip += 2;
				} else {
					self.ip = operand as usize;
				}
			},
			4 => { // bxc: bitwise xor, operand: read but ignored.
				self.reg[REG_B] = self.reg[REG_B] ^ self.reg[REG_C];
				self.ip += 2;
			},
			5 => { // out: output, operand: combo
				let r = self.get_combo_value(operand) & 0x07;
				self.output.push(r);
				self.ip += 2;
			},
			_ => {
				panic!("Unknown instruction {}", instruction);
			},
		};

		true
	}
	pub fn get_combo_value(&self, co: u8) -> u64 {
		match co {
			0..=3 => co as u64,
			4 => self.reg[REG_A],
			5 => self.reg[REG_B],
			6 => self.reg[REG_C],
			_ => panic!("invalid combo operand"),
		}
	}

}

pub fn day17(input: &String) -> (usize, usize) {
	// read in input
	let re = regex::Regex::new(r"(-?\d+)").expect("valid regex");
	let caps: Vec<u64> = re.find_iter(input).map(|m| m.as_str().parse::<u64>().unwrap()).collect_vec();
	println!("{:?}", caps);

	// initialise computer
	let program = caps[3..].iter().map(|&u| u as u8).collect_vec();
	let mut c = Computer::new(program);
	c.reg[0] = caps[0];
	c.reg[1] = caps[1];
	c.reg[2] = caps[2];

	// step until halted
	let mut running: bool = true;
	while running {
		running = c.step();
		println!("ip: {}", c.ip);
		println!("output: {:?}", c.output);
	}

	let mut final_output: String = c.output.iter().map(|u| u.to_string() + ",").collect();
	let final_output = final_output.trim_end_matches(",");
	println!("output: {}", final_output);
	(0,0)
}