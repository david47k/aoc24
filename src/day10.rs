use crate::grid::{*};
use itertools::Itertools;
use crate::vector::{*};
use crate::path::ALLMOVES;
use std::collections::{*};

pub fn day10(input: &String) -> (String,String) {
    // find trails from trailhead (0) to peak (9), incrementing one each time
    // trailhead score is how many 9s are reachable
    // sum of trailhead scores is the answer to part one
    // part two: ratings: number of distinct trails (i.e. different paths that go from 0 to any 9)

    // read input into grid
    let grid = Grid::from_str(input);
    println!("grid w {0} h {1}", grid.w, grid.h);

    let mut scores: Vec<usize> = vec![];
    let mut ratings: Vec<usize> = vec![];

    // find trailheads
    let trailheads = grid.find(b'0');

    // for each trailhead
    for th in trailheads {
        let mut nines: BTreeSet<Vector> = BTreeSet::new();
        let mut unique_hikes: Vec<Vec<Vector>> = vec![];
        path_walk(&grid, vec![th], &mut nines, &mut unique_hikes);
        scores.push(nines.len());
        ratings.push(unique_hikes.len());
    }

    //println!("scores: {0:?}", scores);
    let th_score: usize = scores.iter().sum();
    let th_ratings: usize = ratings.iter().sum();
    println!("part one scores sum: {th_score}");
    println!("part two ratings sum: {th_ratings}");
    (th_score.to_string(), th_ratings.to_string())
}

fn path_walk(grid: &Grid, path: Vec<Vector>, nines: &mut BTreeSet<Vector>, hike_paths: &mut Vec<Vec<Vector>> ) {
    let xy = path.last().unwrap();
    let height = grid.get_unchecked(&xy);

    // if we are at a nine, save the position
    if grid.get(path.last().expect("Vector")).expect("valid Vector") == b'9' {
        nines.insert(xy.clone());
        hike_paths.push(path.clone());
        return;
    }

    // what directions can we go from here?
    let possibles = ALLMOVES.iter().filter(|&m| {
        let nxy = xy.add(&m.to_vector());
        grid.has_xy(&nxy) && (grid.get_unchecked(&nxy) == height + 1)
    }).collect_vec();

    // perform moves
    possibles.iter().for_each(|m| {
        let nxy = xy.add(&m.to_vector());
        let mut npath = path.clone();
        npath.push(nxy);
        path_walk(grid, npath, nines, hike_paths);
    });
}