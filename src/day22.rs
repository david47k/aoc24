use itertools::Itertools;
use std::collections::{*};

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
		//println!("{n}: {}", all_numbers[i][1999]);
		p1result += all_numbers[i][1999];
	}

	println!("part 1 result: {p1result}");

	// part 2
	// we can use all_numbers, all_prices, all_diffs from above

	let t0 = crate::time::get_time_ms();

	// calculate a price diff set, to speed things up
	println!("calculating diff sets for part 2...");
	let mut all_sets: Vec<Vec<[i8;4]>> = vec![];
	let mut buyer_maps: Vec<BTreeMap<[i8;4],i32>> = vec![];
	for buyer_idx in 0..all_prices.len() {
		let mut buyer_map: BTreeMap<[i8;4],i32> = BTreeMap::new();
		let mut set: Vec<[i8;4]> = vec![];
		for i in 0..2000-4_usize {
			let x = [ all_diffs[buyer_idx][i], all_diffs[buyer_idx][i+1], all_diffs[buyer_idx][i+2], all_diffs[buyer_idx][i+3] ];
			set.push(x);
			if !buyer_map.contains_key(&x) {
				// save the price... which is all_prices[buyer_idx][i + 3];
				buyer_map.insert(x, all_prices[buyer_idx][i+3] as i32);
			}
		}
		all_sets.push(set);
		buyer_maps.push(buyer_map);
	}
	println!("done!");

	let mut best_pattern = all_sets[0][0];
	let mut best_price: i32 = 0;

	let mut tested_patterns: BTreeSet<[i8;4]> = BTreeSet::new();


	for buyer_idx in 0..all_prices.len() {
		println!("buyer: {buyer_idx}");
		for i in 0..2000-4_usize {
			let pattern = all_sets[buyer_idx][i];
			if tested_patterns.contains(&pattern) {
				continue;
			}
			tested_patterns.insert(pattern);
			let mut price: i32 = 0;
			for buyer_idx_2 in buyer_idx..all_prices.len() {
				if let Some(j) = buyer_maps[buyer_idx_2].get(&pattern) {
					price += j;
				}
			}
			if price > best_price {
				best_pattern = pattern;
				best_price = price;
			}
		}
		println!("best price so far: {:5}   {:?}", best_price, best_pattern);
		if buyer_idx == 10 {
			break;
		}
	}

	let t1 = crate::time::get_time_ms();

	println!("part 2 result: best price after maximum 10 buyers: {best_price}");
	println!("part 2 time: {:.0} ms", t1 - t0);
	(p1result.to_string(), best_price.to_string())
}