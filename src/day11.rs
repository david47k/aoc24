//use itertools::Itertools;
//use std::collections::{*};
//use crate::grid::{*};
//use crate::vector::{*};

use itertools::Itertools;

#[derive(Clone)]
enum Space {
    Stone(u32),
    Stones(Vec<Space>),
}
pub fn day11(input: &String) {
    // read in numbers
    let input: Vec<Space> = input.trim_end().split_whitespace().map(|s| s.parse::<u32>().expect("number")).map(|n| Space::Stone(n)).collect_vec();
    let mut stones = Space::Stones(input);
    let mut blinks: usize = 0;
    let mut part1_count: usize = 0;
    let mut part2_count: usize = 0;

    let v = flatten_v(&stones);
    println!("{0} blinks, {1} stones", blinks, v.len());

    for b in 1..=75 {
        blinks = b;
        stones = apply_blink(stones);
        let stone_count = count_stones(&stones);
        if b % 5 == 0 {
            println!("flattening...");
            let v = flatten_v(&stones);
            stones = Space::Stones(v.iter().map(|&n| Space::Stone(n)).collect_vec());
        }
        if b == 25 {
            part1_count = stone_count;
        }
        if b == 75 {
            part2_count = stone_count;
        }
        println!("{0} blinks, {1} stones", blinks, stone_count);
    }
    println!("part one: {0}", part1_count);
    println!("part two: {0}", part2_count);

}
fn flatten_v(sp: &Space) -> Vec<u32> {
    match sp {
        Space::Stones(v) => v.iter().map(|x| flatten_v(x)).flatten().collect_vec(),
        Space::Stone(n) => vec![*n],
    }
}

fn count_stones(sp: &Space) -> usize {
    match sp {
        Space::Stones(v) => v.iter().map(|x| count_stones(x)).sum(),
        Space::Stone(_) => 1,
    }
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

fn apply_blink(sp: Space) -> Space {
    match sp {
        Space::Stones(v) => Space::Stones(v.into_iter().map(|x| apply_blink(x)).collect_vec()),
        Space::Stone(n) => {

            if n == 0 {
                return Space::Stone(1);
            } else if !count_digits_odd(n) {
                let s = n.to_string();
                let lh = s[0..s.len() / 2].parse::<u32>().expect("number");
                let rh = s[s.len() / 2..s.len()].parse::<u32>().expect("number");
                return Space::Stones(vec![Space::Stone(lh), Space::Stone(rh)]);
            } else {
                return Space::Stone(n * 2024);
            }
        }
    }
}


