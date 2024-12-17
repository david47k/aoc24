use itertools::Itertools;
//use std::collections::{*};
use crate::vector::{*};
use crate::grid::{*};
use crate::path::ALLMOVES;

pub fn day12(input: &String) -> (usize,usize) {
    // read into grid
    let grid = Grid::from_str(input);
    println!("grid w {0} h {1}", grid.w, grid.h);

    // need to find regions, each with area and perimeter
    // input is A-Z
    let mut total_price: usize = 0;
    let mut total_discount_price: usize = 0;
    for &crop in b"ABCDEFGHIJKLMNOPQRSTUVWXYZ" {
        // find a crop region
        let crop_spots = grid.find(crop);
        let mut visited: Vec<Vector> = vec![];
        for &xy in &crop_spots {
            if visited.contains(&xy) { continue; }
            let mut area: usize = 1;
            let mut perimeter: usize = 0;
            visited.push(xy);
            let mut this_region: Vec<Vector> = vec![xy];
            let ap = check_surrounds(&grid, &mut this_region, crop, xy);
            let corners = count_corners(&grid, crop, &this_region);
            area += ap.area;
            perimeter += ap.perimeter;
            let price = area * perimeter;
            let discount_price = area * corners;
            println!("crop {} at xy {} has area {} and perimeter {} costing ${}", crop as char, xy.to_string(), area, perimeter, price);
            println!("    it has {} corners and discounted price ${}", corners, discount_price);
            total_price += price;
            total_discount_price += discount_price;
            visited.append(&mut this_region);
        }
    }
    println!("Total price is {}", total_price);
    println!("Total discounted price is {}", total_discount_price);
    (total_price,total_discount_price)
}

fn count_corners(grid: &Grid, crop: u8, this_region: &Vec<Vector>) -> usize {
    let mut count: usize = 0;

    for xy in this_region {
        //  C!  this is a concave corner
        //  !x
        //   
        //  Cc
        //  c!  this is a convex corner

        let nbs: Vec<Option<u8>> = grid.get_neighbours(&xy);
        // convert bool to u8 here because we can't index on a Vec<bool> (!)
        let nbsc: Vec<u8> = nbs.iter().map(|c| (c.is_some() && (c.unwrap() == crop)) as u8).collect_vec();

        let regions = [ 
            [ nbsc[NDIR_R], nbsc[NDIR_D], nbsc[NDIR_DR] ],
            [ nbsc[NDIR_L], nbsc[NDIR_D], nbsc[NDIR_DL] ],
            [ nbsc[NDIR_R], nbsc[NDIR_U], nbsc[NDIR_UR] ],
            [ nbsc[NDIR_L], nbsc[NDIR_U], nbsc[NDIR_UL] ],
        ];

        for &[a, b, c] in regions.iter() {
            let (a, b, c) = (a!=0, b!=0, c!=0); // convert u8 to bool
            if (!a && !b) || (a && b && !c) {
                count += 1;
            } 
        } 
    }
    count
}

struct AP {
    area: usize,
    perimeter: usize,
}
fn check_surrounds(grid: &Grid, visited: &mut Vec<Vector>, crop: u8, xy: Vector) -> AP
{
    let mut perimeter: usize = 0;
    let mut area: usize = 0;
    // should already have done:
    // visited.push(xy)
    // area += 1
    // perimeter = 0

    ALLMOVES.iter().for_each(|m| {
        let nxy = xy.add(&m.to_vector());
        if !grid.has_xy(&nxy) || crop != grid.get(&nxy).unwrap() {
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
    AP { area, perimeter }
}