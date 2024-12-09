use crate::grid::{*};
use itertools::Itertools;

pub fn day08(input: &String) {

    // nodes are a-zA-Z0-9
    // antinodes are created in two directions from each pair of matching nodes\
    // how many antinodes are there (on the map)?

    // read input into grid
    let grid = Grid::from_str(input);
    println!("grid w {0} h {1}", grid.w, grid.h);

    let mut antinodes = vec![];

    // for each node possibility, find all nodes
    for node in b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"[..].iter()  {
        let positions = grid.find(*node);
        if positions.len() > 0 {
            print!("'{0}' at ", *node as char);
            for p in &positions {
                print!("{0} ", p.to_string());
            }
            println!();
            // find antinodes
            // i.e. for each pair of the set, find the two antinodes
            for pair in positions.iter().combinations(2) {
                println!("combo {0} {1}", pair[0].to_string(), pair[1].to_string());
                let d = pair[1].sub(pair[0]);
                let an0 = pair[0].sub(&d);
                let an1 = pair[1].add(&d);
                println!("antinodes at {0} {1}", an0.to_string(), an1.to_string());
                antinodes.push(an0);
                antinodes.push(an1);
            }
        }

    }

    // count how many unique ones that are on the map
    antinodes.sort();
    antinodes.dedup();
    let antinodes = antinodes.iter().filter(|n| n.is_valid(&grid)).collect_vec();
    println!("part one: unique antinodes: {}", antinodes.len());

    // part two
    // antinodes also occur at pairs, and at every multiple of the spacing

    let mut antinodes = vec![];

    // for each node possibility, find all nodes
    for node in b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"[..].iter()  {
        let positions = grid.find(*node);
        if positions.len() > 0 {
            print!("'{0}' at ", *node as char);
            for p in &positions {
                print!("{0} ", p.to_string());
            }
            println!();
            // find antinodes
            // i.e. for each pair of the set, find the antinodes in both directions until off-map
            for pair in positions.into_iter().combinations(2) {
                println!("combo {0} {1}", pair[0].to_string(), pair[1].to_string());
                let d = pair[1].sub(&pair[0]);
                antinodes.push(pair[0]);
                antinodes.push(pair[1]);

                // first direction
                let mut an0 = pair[0].sub(&d);
                while grid.has_xy(&an0) {
                    antinodes.push(an0);
                    an0 = an0.sub(&d);
                }

                // second direction
                let mut an1 = pair[1].add(&d);
                while grid.has_xy(&an1) {
                    antinodes.push(an1);
                    an1 = an1.add(&d);
                }
            }
        }

    }

    // count how many unique ones that are on the map
    antinodes.sort();
    antinodes.dedup();
    println!("part two: unique antinodes: {}", antinodes.len());
}