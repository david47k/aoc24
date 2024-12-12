use itertools::Itertools;
//use std::collections::{*};
use crate::grid::{*};
use crate::vector::{*};

use crate::grid::Grid;

pub fn day12(input: &String) {
    // read into grid
    let grid = Grid::from_str(input);
    println!("grid w {0} h {1}", grid.w, grid.h);

    // need to find regions, each with area and perimeter
    // input is A-Z
    let mut total_price: usize = 0;
    for &crop in b"ABCDEFGHIJKLMNOPQRSTUVWXYZ" {
        // find a crop region
        let crop_spots = grid.find(crop);
        let mut visited: Vec<XY> = vec![];
        for xy in crop_spots {
            if visited.contains(&xy) { continue; }
            let mut area: usize = 1;
            let mut perimeter: usize = 0;
            visited.push(xy);
            let ap = check_surrounds(&grid, &mut visited, crop, xy);
            area += ap.area;
            perimeter += ap.perimeter;
            let price = area * perimeter;
            println!("crop {} at xy {} has area {} and perimeter {} costing ${}", crop as char, xy.to_string(), area, perimeter, price);
            total_price += price;
        }
    }
    println!("Total price is {}", total_price);
}

// check surroundings

struct AP {
    area: usize,
    perimeter: usize,
}
fn check_surrounds(grid: &Grid, visited: &mut Vec<XY>, crop: u8, xy: XY) -> AP
{
    let mut perimeter: usize = 0;
    let mut area: usize = 0;
    // should already have done:
    // visited.push(xy)
    // area += 1
    // perimeter = 0

    ALLMOVES.iter().for_each(|m| {
        let nxy = xy.add(&m.to_xy());
        if !grid.has_xy(&nxy) {
            perimeter += 1;
            return; // from closure
        }
        let ncrop = grid.get(&nxy).unwrap();
        if ncrop != crop {
            perimeter += 1;
            return; // from closure
        }
        if visited.contains(&nxy) {
            return; // from closure
        }
        // 'visit' this spot
        visited.push(nxy);
        area += 1;
        let npa = check_surrounds(grid, visited, crop, nxy);
        perimeter += npa.perimeter;
        area += npa.area;
    });
    return AP { area, perimeter };
}