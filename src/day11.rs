//use itertools::Itertools;
//use std::collections::{*};
//use crate::grid::{*};
//use crate::vector::{*};

use itertools::Itertools;

pub fn day11(input: &String) {
    // read in numbers
    let mut stones: Vec<u32> = input.trim_end().split_whitespace().map(|s| s.parse::<u32>().expect("number")).collect_vec();

    let mut blinks: usize = 0;
    let mut part1_count: usize = 0;
    let mut part2_count: usize = 0;

    let t0 = crate::time::get_time_ms();
    let t1: f64 = 0_f64;

    //println!("{0} blinks, {1} stones", blinks, v.len());

    for i in 0..75 {
        println!("depth {0}", i+1);
        let len = stones.len();
        for si in 0..len {
            let extra = apply_blink(&mut stones[si]);
            if let Some(x) = extra {
                stones.push(x);
            }
        }
        println!("stones {0}", stones.len());

        if i == 24 {
            part1_count = stones.len();
            println!("part one: {0}", part1_count);
        }
        if i == 37 {
            println!("--------------------\ntime: {0:4.3}s\n--------------------", (crate::time::get_time_ms() - t0)/1000_f64);
        }
        if i == 74 {
            part2_count = stones.len();
        }
    }

    println!("part one: {0}", part1_count);
    println!("part two: {0}", part2_count);

}

fn count_digits_odd(n: u32) -> bool {
    // return true if number of digits is odd
    if n < 10 {
        return true;
    }
    if n < 100 {
        return false;
    }
    if n < 1000 {
        return true;
    }
    if n < 10000 {
        return false;
    }
    if n < 100000 {
        return true;
    }
    if n < 1000000 {
        return false;
    }
    if n < 10000000 {
        return true;
    }
    if n < 100000000 {
        return false;
    }
    if n < 1000000000 {
        return true;
    }
    return false;
}

fn apply_blink(n: &mut u32) -> Option<u32> {
    if *n == 0 {
        *n = 1;
        return None;
    }
    if !count_digits_odd(*n) {
        let s = n.to_string();
        let lh = s[0..s.len() / 2].parse::<u32>().expect("number");
        let rh = s[s.len() / 2..s.len()].parse::<u32>().expect("number");
        *n = lh;
        return Some(rh);
    }
    *n = *n * 2024;
    return None;
}

