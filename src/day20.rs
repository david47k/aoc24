use itertools::Itertools;
use std::collections::{*};
//use crate::grid::{*};
use crate::vector::{*};
use crate::level::{*};
use crate::path2::{*};
use crate::solve::{*};

pub fn day20(input: &String) -> (String, String) {
    // find path... with cheats!
    // this question was poorly worded (in fact the example cheats don't match the written description of the cheats...)
    // first find optimal path
    let mut level = Level::from_str(input).expect("valid level");
    let mut best_score: u64 = 0;
    let mut best_path_pts: Vec<Vector> = vec![];
    let mut best_solution;
    println!("level w: {}, h: {}", level.w, level.h);
    println!("{}", level.to_string());
    let max_depth: u64 = crate::stackstack::STACKSTACK64_MAX as u64 * 32;
    let soln = find_best_path_18(&level, max_depth, None);

    if let Some(ref sol) = soln {
        println!("Basic solution found!");
        best_solution = sol.clone();
        best_path_pts = sol.visited.clone();
        //let ss: String = sol.path.iter().map(|m| m.to_string()).collect();
        println!("len of best path pts: {}", best_path_pts.len());
        println!("len of path: {}", sol.path.len());
        //println!("solution : {}", ss);
        best_score = sol.score as u64;
        println!("score: {}", best_score);
    } else {
        println!("no solution");
    }

    // try different cheats... looks like we can basically remove 1 wall somewhere...
    // BUT the 1 wall has to be next to the original path, and you end up on the original path!

    let mut p1count = 0_u64;
    for (i,pt) in best_path_pts.iter().enumerate() {
        for m in ALLMOVES2 {
            let magic1 = pt.apply_dir(&m);
            let magic2 = magic1.apply_dir(&m);
            if !level.vector_in_bounds(&magic2) {
                continue;
            }
            if !level.wall_bmp.get_v(magic1) || level.wall_bmp.get_v(magic2) {
                continue;
            }
            // check magic2 is on path, in a later place than magic1
            if !best_path_pts.iter().skip(i).contains(&magic2) {
                continue;
            }
            // find index of magic2
            let i2 = best_path_pts.iter().skip(i).position(|&v| v==magic2).unwrap() + i;
            let difference = i2 - i - 2;

            // do we have a quicker solution here?
            // println!("Cheat found at {:3},{:3} and {:3},{:3}. Difference {:3}.", magic1.0, magic1.1, magic2.0, magic2.1, difference);
            if difference >= 100 {
                p1count += 1;
            }
        }
    }

    println!("part 1 count: {}", p1count);

    ("no result".to_string(), "no result".to_string())
}