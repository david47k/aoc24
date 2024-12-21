// Box Bopper: Sokoban-like game
// Copyright David Atkinson 2020-2021
//
// stackstack.rs: a stack on the stack, used to speed up inner loops by avoiding memory allocation
//
// default multiplier is 1 (512 bits or path of 256) but it'll overflow with levels which can have long paths

pub const STACKSTACK64_MAX: usize = 320;	// this is a fairly optimised value for day 20 part 1

#[derive(PartialOrd, Eq, Ord, Clone, Copy, PartialEq, Debug)]
pub struct StackStack64 {
	pub next: usize,
	pub stack: [u64; STACKSTACK64_MAX],
}

impl StackStack64 {
	pub fn new() -> StackStack64 {
		StackStack64 {
			next: 0,
			// this is an optimisation, we don't init the stack as we will overwrite any data in it as necessary
			#[allow(invalid_value)]
			stack: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
		}
	}
	pub fn push(&mut self, d: u64) {
        if self.next == STACKSTACK64_MAX { panic!("StackStack64 overflow"); }
        self.stack[self.next] = d;
		self.next += 1;
	}
	pub fn pop(&mut self) -> u64 {
        if self.next == 0 { panic!("StackStack64 underflow"); }
		self.next -= 1;
		self.stack[self.next]
	}
	pub fn len(&self) -> usize {
		self.next
	}
	pub fn clear(&mut self) {
		self.next = 0;
	}
}
