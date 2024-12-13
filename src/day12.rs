use itertools::Itertools;
//use std::collections::{*};
use crate::grid::{*};
use crate::vector::{*};

use crate::grid::Grid;

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
        let mut visited: Vec<XY> = vec![];
        for &xy in &crop_spots {
            if visited.contains(&xy) { continue; }
            let mut fenced_sides = Grid::new(grid.w, grid.h);     // for part 2: save sides!
            let mut area: usize = 1;
            let mut perimeter: usize = 0;
            visited.push(xy);
            let ap = check_surrounds(&grid, &mut visited, crop, xy, &mut fenced_sides);
            area += ap.area;
            perimeter += ap.perimeter;
            let price = area * perimeter;
            let corners = count_corners(&fenced_sides, &visited);
            let discount_price = area * corners;
            println!("crop {} at xy {} has area {} and perimeter {} costing ${}", crop as char, xy.to_string(), area, perimeter, price);
            println!("    it has {} corners and discounted price ${}", corners, discount_price);
            total_price += price;
            total_discount_price += discount_price;
        }
    }
    println!("Total price is {}", total_price);
    println!("Total discounted price is {}", total_discount_price);
    (total_price,total_discount_price)
}

fn count_corners(grid: &Grid, visited: &Vec<XY>) -> usize {
    let mut count: usize = 0;
    for y in 0..grid.h {
        for x in 0..grid.w {
            let sides = grid.get_unchecked(&XY::new(x,y));
            // there are 16 total combinations, only some are corners
            let options = [
                (DIR_U|DIR_L, 1),
                (DIR_U|DIR_R, 1),
                (DIR_U|DIR_L | DIR_R, 2),
                (DIR_L|DIR_U | DIR_D, 2),
                (DIR_L | DIR_D, 1),
                (DIR_L | DIR_D | DIR_R, 2),
                (DIR_D | DIR_R, 1),
                (DIR_R | DIR_U | DIR_D, 2),
                (DIR_U | DIR_D | DIR_L | DIR_R, 4),
            ];
            count += options.iter()
                .find(|(calc, _)| *calc == sides)
                .map(|(_, ret)| *ret)
                .unwrap_or(0);
        }
    }
    println!("corner count part 1: {}", count);

    // look for convex corners
    for y in 0..grid.h {
        for x in 0..grid.w {
            let xy = XY::new(x,y);
            let sides = grid.get_unchecked(&XY::new(x,y));
            // we can only check spots that are NOT in our crop_spots
            let pts = [ &XY::new(-1,-1), &XY::new(1,-1), &XY::new(-1,1), &XY::new(1,1) ];
            let pts = pts.iter().map(|&pt| xy.add(pt)).collect_vec();
            let mut TL = grid.get(&pts[0]);
            let mut TR = grid.get(&pts[1]);
            let mut BL = grid.get(&pts[2]);
            let mut BR = grid.get(&pts[3]);
            let pts2 = [ &XY::new(0,-1), &XY::new(1,0), &XY::new(0,1), &XY::new(-1,0) ];
            if TL.is_some() {
                // check if ABOVE and LEFT of x,y is NOT in visited.. to avoid double-counting touching corners
                if !visited.contains(&xy.add(&pts2[0])) && !visited.contains(&xy.add(&pts2[3])) { TL = None; }
            }
            if TR.is_some() {
                if !visited.contains(&xy.add(&pts2[0])) && !visited.contains(&xy.add(&pts2[1])) { TR = None; }
            }
            if BL.is_some() {
                if !visited.contains(&xy.add(&pts2[2])) && !visited.contains(&xy.add(&pts2[3])) { BL = None; }
            }
            if BR.is_some() {
                if !visited.contains(&xy.add(&pts2[2])) && !visited.contains(&xy.add(&pts2[1])) { BR = None; }
            }

            // there are 16 total combinations, only some are corners
            if let Some(xtl) = TL {
                if (sides & DIR_U) > 0 && (xtl & DIR_R) > 0 {
                    count += 1;
                }
            }
            if let Some(xtr) = TR {
                if (sides & DIR_U) > 0 && (xtr & DIR_L) > 0 {
                   count += 1;
                }
            }
            if let Some(xbl) = BL {
                if (sides & DIR_D) > 0 && (xbl & DIR_R) > 0 {
                    count += 1;
                }
            }
            if let Some(xbr) = BR {
                if (sides & DIR_D) > 0 && (xbr & DIR_L) > 0 {
                    count += 1;
                }
            }
        }
    }
    println!("corner count part 2: {}", count);

    count
}

struct AP {
    area: usize,
    perimeter: usize,
}
fn check_surrounds(grid: &Grid, visited: &mut Vec<XY>, crop: u8, xy: XY, fenced_sides: &mut Grid) -> AP
{
    let mut perimeter: usize = 0;
    let mut area: usize = 0;
    // should already have done:
    // visited.push(xy)
    // area += 1
    // perimeter = 0

    ALLMOVES.iter().for_each(|m| {
        let nxy = xy.add(&m.to_xy());
        if !grid.has_xy(&nxy) || crop != grid.get(&nxy).unwrap() {
            perimeter += 1;
            let fs = fenced_sides.get_unchecked(&xy);
            fenced_sides.put(&xy, (*m as u8) | fs);
            return; // from closure
        }
        if visited.contains(&nxy) {
            return; // from closure
        }
        // 'visit' this spot
        visited.push(nxy);
        area += 1;
        let npa = check_surrounds(grid, visited, crop, nxy, fenced_sides);
        perimeter += npa.perimeter;
        area += npa.area;
    });
    AP { area, perimeter }
}