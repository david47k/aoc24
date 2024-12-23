use itertools::Itertools;
//use std::collections::{*};
//use crate::grid::{*};
//use crate::vector::{*};

fn secret_number_step(mut input: u64) -> u64 {
	let x = input * 64;
	input ^= x;
	input &= 0xFFFFFF;

	let x = input / 32;
	input ^= x;
	input &= 0xFFFFFF;

	let x = input * 2048;
	input ^= x;
	input &= 0xFFFFFF;
	input
}

pub fn day22(input: &String) -> (String, String) {
	let initial_numbers = input.lines().map(|s| s.parse::<u64>().unwrap()).collect_vec();

	let mut all_numbers: Vec<Vec<u64>> = vec![];
	let mut all_prices: Vec<Vec<i8>> = vec![];
	let mut all_diffs: Vec<Vec<i8>> = vec![];

	let mut p1result: u64 = 0;

	for (i,n) in initial_numbers.iter().enumerate() {
		let mut prices: Vec<i8> = vec![];
		let mut diffs: Vec<i8> = vec![];
		let mut numbers: Vec<u64> = vec![];
		let mut result: u64 = *n;
		let mut pprice: i8 = (result%10) as i8;
		for _i in 0..2000 {
			result = secret_number_step(result);

			numbers.push(result);
			let price = (result%10) as i8;
			prices.push(price);
			diffs.push(price-pprice);
			pprice = price;
		}
		all_numbers.push(numbers);
		all_prices.push(prices);
		all_diffs.push(diffs);
		println!("{n}: {}", all_numbers[i][1999]);
		p1result += all_numbers[i][1999];
	}

	println!("part 1 result: {p1result}");

	// part 2
	// we can use all_numbers, all_prices, all_diffs from above
	let mut best_pattern = &all_diffs[0][0..4];
	let mut best_price: i64 = 0;

	for buyer_idx in 0..all_prices.len() {
		println!("buyer: {buyer_idx}");
		for i in 0..2000-4_usize {
			let pattern = &all_diffs[buyer_idx][i..i + 4];
			let mut price: i64 = 0;
			for buyer_idx_2 in buyer_idx..all_prices.len() {
				for j in 0..2000 - 4_usize {
					// if i == 0 && j < 10 {
					// 	println!("{:10}: {} {}", all_numbers[i][j], all_prices[i][j], all_diffs[i][j]);
					// }

					if *pattern == all_diffs[buyer_idx_2][j..j + 4] {
						price += all_prices[buyer_idx_2][j + 3] as i64;
						break;	// where the pattern FIRST occurs
					}
				}
			}
			if price > best_price {
				best_pattern = pattern;
				best_price = price;
			}
		}
		println!("best price so far: {best_price}");
		println!("best pattern so far: {:?}", best_pattern);
	}


	(p1result.to_string(), "no result".to_string())

}