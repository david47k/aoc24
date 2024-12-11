use std::collections::BTreeMap;
use itertools::Itertools;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Num(usize);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Node {
    count: usize,
    children: Option<(Num, Option<Num>)>,
}

impl Node {
    fn to_string(&self) -> String {
        format!("{0}", self.count)
    }
}

fn blink_node(src: &BTreeMap<Num,Node>, dest: &mut BTreeMap<Num,Node>, n: Num) {
    let node = src.get(&n).unwrap();

<<<<<<< Updated upstream
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
=======
    // calculate the children, if they don't exist
    if node.children.is_none() {
        dest.get_mut(&n).unwrap().children = Some(calculate_blink(n));
    }
>>>>>>> Stashed changes

    // increment the children
    let count = node.count;
    let children = dest.get_mut(&n).unwrap().children.clone().unwrap();

    if dest.contains_key(&children.0) {
        dest.get_mut(&children.0).unwrap().count += count;
    } else {
        dest.insert(children.0, Node { count: count, children: None });
    }

    if let Some(c1) = children.1 {
        if dest.contains_key(&c1) {
            dest.get_mut(&c1).unwrap().count += count;
        } else {
            dest.insert(c1, Node { count: count, children: None });
        }
    }

    // decrement ourselves
    dest.get_mut(&n).unwrap().count -= count;
}

<<<<<<< Updated upstream
fn count_digits_odd(n: u32) -> bool {
    // return true if number of digits is odd
=======
fn calculate_count(map: &BTreeMap<Num, Node>) -> usize {
    map.iter().map(|(_, n)| n.count).sum::<usize>()
}

pub fn day11(input: &String) {
    // read in numbers
    let input: Vec<usize> = input.trim_end().split_whitespace().map(|s| s.parse::<usize>().expect("number")).collect_vec();

    let mut map: BTreeMap<Num, Node> = BTreeMap::new();

    for s in input {
        map.insert(Num(s), Node { count: 1, children: None });
    }

    let mut part1_count: usize = 0;
    let mut part2_count: usize = 0;
    let t0 = crate::time::get_time_ms();
    let mut blinks : usize;

    for i in 0..75 {
        let mut dest = map.clone();
        for &k in map.keys() {
            blink_node(&map, &mut dest, k);
        }
        map = dest;
        blinks = i + 1;

        println!("depth {blinks}, count {0}", calculate_count(&map));
        // print!("nodes: ");
        // map.iter().for_each(|(k,n)| {
        //     print!("{0}:{1} ", k.0, n.to_string());
        // });
        // println!();

        if blinks == 25 {
            part1_count = calculate_count(&map);
        }
        if blinks == 75 {
            part2_count = calculate_count(&map);
        }
    }

    println!("time: {0:4.3}s", (crate::time::get_time_ms() - t0)/1000_f64);
    println!("part one: {part1_count}");
    println!("part two: {part2_count}");
}

fn count_digits(n: usize) -> (usize, bool) {
    // return true if number of digits is even
>>>>>>> Stashed changes
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
<<<<<<< Updated upstream
    return false;
=======
    if n < 10000000000 {
        return (10, true);
    }
    if n < 100000000000 {
        return (11, false);
    }
    if n < 1000000000000 {
        return (12, true);
    }
    panic!("count_digits input is too high!");
>>>>>>> Stashed changes
}

fn calculate_blink(n: Num) -> (Num, Option<Num>) {
    if n.0 == 0 {
        return (Num(1), None);
    }
<<<<<<< Updated upstream
    if !count_digits_odd(*n) {
        let s = n.to_string();
        let lh = s[0..s.len() / 2].parse::<u32>().expect("number");
        let rh = s[s.len() / 2..s.len()].parse::<u32>().expect("number");
        *n = lh;
        return Some(rh);
=======
    let c = count_digits(n.0);
    if c.1 {
        let (lh,rh) = split_num(n.0, c.0);
        return (Num(lh),Some(Num(rh)));
>>>>>>> Stashed changes
    }
    return (Num(n.0 * 2024), None);
}

<<<<<<< Updated upstream
=======
fn split_num(n: usize, c: usize) -> (usize, usize) {
    // split n into two, if number of digits is even
    // c is count of digits
    match c {
        2 =>  (n/10,      n%10),
        4 =>  (n/100,     n%100),
        6 =>  (n/1000,    n%1000),
        8 =>  (n/10000,   n%10000),
        10 => (n/100000,  n%100000),
        12 => (n/1000000, n%1000000),
        _ => panic!("expect even number of digits <= 12"),
    }
}
>>>>>>> Stashed changes
