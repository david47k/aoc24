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

    // calculate the children, if they don't exist
    if node.children.is_none() {
        dest.get_mut(&n).unwrap().children = Some(calculate_blink(n));
    }

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

fn calculate_count(map: &BTreeMap<Num, Node>) -> usize {
    map.iter().map(|(_, n)| n.count).sum::<usize>()
}

pub fn day11(input: &String) -> (usize,usize) {
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
    (part1_count, part2_count)
}

fn count_digits(n: usize) -> (usize, bool) {
    // return true if number of digits is even
    if n < 10 {
        return (1, false);
    }
    if n < 100 {
        return (2, true);
    }
    if n < 1000 {
        return (3, false);
    }
    if n < 10000 {
        return (4, true);
    }
    if n < 100000 {
        return (5, false);
    }
    if n < 1000000 {
        return (6, true);
    }
    if n < 10000000 {
        return (7, false);
    }
    if n < 100000000 {
        return (8, true);
    }
    if n < 1000000000 {
        return (9, false);
    }
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
}

fn calculate_blink(n: Num) -> (Num, Option<Num>) {
    if n.0 == 0 {
        return (Num(1), None);
    }
    let c = count_digits(n.0);
    if c.1 {
        let (lh,rh) = split_num(n.0, c.0);
        return (Num(lh),Some(Num(rh)));
    }
    return (Num(n.0 * 2024), None);
}

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
