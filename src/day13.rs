use itertools::Itertools;
//use std::collections::{*};
//use crate::grid::{*};
//use crate::vector::{*};
//use crate::vector::Vector;

struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    t: (usize, usize),
}

struct Combo {
    a_count: usize,
    b_count: usize,
    cost: usize,
}
pub fn day13(input: &String) -> (usize,usize) {
    let mut total_cost: usize = 0;
    let input = input.lines().collect_vec();
    let machine_count = (input.len()+1)/ 4;
    println!("machine_count: {}", machine_count);
    let mut machines: Vec<Machine> = Vec::with_capacity(machine_count);

    // read in all the data
    for m in 0..machine_count {
        let lines = &input[(m * 4)..(m * 4 + 3)];
        let ax: usize = lines[0][12..14].parse::<usize>().unwrap();
        let ay: usize = lines[0][18..20].parse::<usize>().unwrap();
        let bx: usize = lines[1][12..14].parse::<usize>().unwrap();
        let by: usize = lines[1][18..20].parse::<usize>().unwrap();
        let re = regex::Regex::new(r"X=(\d+), Y=(\d+)").expect("valid regex");
        let caps: [&str; 2] = re.captures(lines[2]).expect("captures").extract().1;
        let t_x = caps[0].parse::<usize>().unwrap();
        let t_y = caps[1].parse::<usize>().unwrap();
        machines.push(Machine { a: (ax, ay), b: (bx, by), t: (t_x, t_y) });
    }

    for (i, m) in machines.iter().enumerate() {
        // find minimum number of button presses that will get close to target
        println!("machine {}: ", i+1);
        println!("  {:?} {:?} {:?}", m.a, m.b, m.t);
        let mut combos: Vec<Combo> = vec![];
        for na in 1..=100 {
            for nb in 1..=100 {
                if (m.a.0 * na + m.b.0 * nb) == m.t.0 && (m.a.1 * na + m.b.1 * nb) == m.t.1 {
                    combos.push(Combo { a_count: na, b_count: nb, cost: 3 * na + nb });
                }
            }
        }
        println!("  {} combos found", combos.len());
        if combos.len() > 0 {
            combos.sort_by(|a, b| a.cost.cmp(&b.cost));
            println!("  cheapest is a: {}, b: {}, cost: {}", combos[0].a_count, combos[0].b_count, combos[0].cost);
            total_cost += combos[0].cost;
        }
    }

    println!("part one total cost: {total_cost}");

    // part two
    let mut total_cost_two: usize = 0;
    for m in machines.iter_mut() {
        m.t = (m.t.0 + 10000000000000, m.t.1 + 10000000000000);
    }

    for (i,m) in machines.iter().enumerate() {
        // can't brute force for part two :)
        // consider the direction each button makes as a line.
        // there are only two possibilities for the lines: line a from (0,0), and line b via (tx,ty) (and vice-versa).
        // the solution to the problem will be the same regardless, as the lines will have the same length to reach
        // the intersection point.
        //
        // we'll calculate the intersection point, then check we can get there in an integer multiple of the button move.
        // we are using floats here, but could also use e.g. the Fraction crate

        println!("machine {}: ", i+1);
        println!("  {:?} {:?} {:?}", m.a, m.b, m.t);

        let a = (m.a.0 as f64, m.a.1 as f64);
        let b = (m.b.0 as f64, m.b.1 as f64);
        let t = (m.t.0 as f64, m.t.1 as f64);

        let ma = a.1 / a.0;                 // gradient of line a
        let mb = b.1 / b.0;                 // gradient of line b
        let cb = t.1 - (b.1/b.0) * t.0;     // y-intercept of line b
        let xi = (ma - mb) / cb;            // x of intercept point
        let yi = ma  * cb / (ma - mb);      // y of intercept point
        let ac = ((yi/a.1).round()) as usize;     // how many times to press button a
        let bc = ((t.1-yi)/b.1).round() as usize; // how many times to press button b

        // println!("  xi: {xi}, yi: {yi}, cb: {cb}, ac: {ac}, bc: {bc}");

        // test the answer to see if it is legit in integer terms
        if (m.a.0 * ac + m.b.0 * bc) == m.t.0 && (m.a.1 * ac + m.b.1 * bc) == m.t.1 {
            let cost = 3 * ac + bc;
            println!("  solution is a: {}, b: {}, cost: {}", ac, bc, cost);
            total_cost_two += cost;
        } else {
            println!("  no solution found");
        }
    }

    println!("part one total cost: {total_cost}");
    println!("part two total cost: {total_cost_two}");
    (total_cost, total_cost_two)
}