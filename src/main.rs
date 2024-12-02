// Advent of Code 2024
// By david47k at d47 dot co

fn main() {
    println!("Advent of Code 2024");
    println!("By david47k at d47 dot co");
    
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 3 {
        println!("Specify which day and input file as arguments (e.g. aoc24 1 ex01.txt)");
        return;
    }
    
    let day: usize = args[1].parse().expect("Day should be an unsigned integer");
    let fname: String = args[2].parse().expect("Filename should be a string");
    let input: String = std::fs::read_to_string(fname).expect("Should be able to read file");
    
    println!("Running day {day}:");
    
    match day {
        1  => day01(&input),
        2  => day02(&input),
        _  => println!("Unknown day!"),
    };
}

fn day01(input: &String) {
    // split input by whitespace, and convert to unsigned integers
    let input: Vec<usize> = input.split_whitespace().map(|s| s.parse::<usize>().expect("Input should be unsigned integers")).collect();
    
    // two seperate vecs for left column and right column
    // could also use transpose, but that's not in std
    let mut a = Vec::<usize>::new();
    let mut b = Vec::<usize>::new();

    // split input into the two seperate vecs
    input.into_iter().enumerate().for_each(|(i,n)| {
        if i%2==0 { 
            a.push(n);
        } else {
            b.push(n);
        }
    });

    // sort!
    a.sort();
    b.sort();
   
    // find differences and sum them
    let sum = a.iter().enumerate().map(|(i,n)| n.abs_diff(b[i])).sum::<usize>();
    
    // solution to part one
    println!("sum: {sum}");

    // part two

    // to start, count how often each number appears in list b    
    // store the count result in a BTreeMap for easy access
    let mut map = std::collections::BTreeMap::<usize,usize>::new();
    b.into_iter().for_each(|n| {
        // if key exists, add to its value. otherwise, insert it with a value of 1
        match map.get_key_value(&n) {
            Some((&k,&v)) => {
                map.insert(k, v+1);
            },
            None => {
                map.insert(n, 1);
            },
        };
    });

    // now iterate through list a, collecting 'similarity scores', and total them
    let score: usize = a.iter().map(|n| {
        match map.get_key_value(&n) {
            Some((&k,&v)) => k * v,
            None => 0,
        }
    }).sum();

    // solution to part two
    println!("score: {score}");
}

fn day02(input: &String) {
    // split input by whitespace, and convert to unsigned integers    
    let reports = input.split('\n').collect::<Vec::<&str>>();
    let reports: Vec<Vec<usize>> = reports.into_iter().map(|r| r.split_whitespace().collect::<Vec::<&str>>().into_iter().map(|s| s.parse::<usize>().expect("Input should be unsigned integers")).collect()).collect();

    // determine if report is safe according to rules
    fn is_safe(r: &Vec<usize>) -> bool {    
        let up = if r[0] < r[1] {
            true
        } else {
            false
        };
        for i in 0..r.len()-1 {
            if r[i] == r[i+1] {
                return false;
            }
            if up {
                if r[i] > r[i+1] {
                    return false;
                }
                let d = r[i+1] - r[i];
                if d > 3 {
                    return false;
                }
            } else {
                if r[i] < r[i+1] {
                    return false;
                }
                let d = r[i] - r[i+1];
                if d > 3 {
                    return false;
                }
            }
        }
    
        true
    }

    // how many reports are safe    
    let mut safe = 0;
    for r in reports.iter() {
        if is_safe(&r) {
            safe += 1;
        }
    }

    println!("safe: {safe}");

    // part two

    fn create_variants(r: &Vec<usize>) -> Vec<Vec<usize>> {
        let mut vs = vec![];
        for i in 0..r.len() {
            let mut v = r.clone();
            v.remove(i);
            vs.push(v);
        }
        vs
    }

    let mut safe2 = 0;
    for r in reports.iter() {
        if is_safe(&r) {
            safe2 += 1;
            continue;
        }
        // create variants of the report
        let vs = create_variants(&r);
        for v in vs.iter() {
            if is_safe(&v) {
                safe2 += 1;
                break;
            }
        }
    }

    println!("part two: {safe2}");
}



