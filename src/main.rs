// Advent of Code 2024
// By david47k at d47 dot co
use regex;

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
        3  => day03(&input),
        4  => day04(&input),
        5  => day05(&input),
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
    let safe = reports.iter().filter(|&r| is_safe(r)).count();

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

fn day03(input: &String) {
    // this looks like a regex challenge first!
    // we'll extract the text group first (easier for debugging)
    // later we might extract the number groups
    let re = regex::Regex::new(r"mul\(\d{1,3},\d{1,3}\)").expect("should be a valid regex");
    let muls: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();

    // extract numbers, multiply, sum results
    let mut sum: usize = 0;
    muls.iter().for_each(|m| {
        // manually finding the numbers
        let i: usize = m.find(',').expect("should be a comma in the map command");        
        let a: usize = m[4..i].parse().expect("should be a number");
        let j: usize = m.find(')').expect("should be a close bracket");
        let b: usize = m[i+1..j].parse().expect("should be a number");
        let r = a * b;
        sum += r;
    });
    println!("sum: {sum}");

    // part two

    // this time we'll extract the do() and don't() instructions as well
    let re = regex::Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))").expect("should be a valid regex");
    let muls: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();

    // extract numbers, multiply, sum results
    let mut sum2: usize = 0;
    let mut enabled = true;
    let re2 = regex::Regex::new(r"(\d{1,3}),(\d{1,3})").expect("valid regex");
    muls.iter().for_each(|m| {
        //match the 4th character -- will be ', ), or (, for don't(), do() and mul() respectively
        match m.chars().nth(3).expect("should be chars-able") {
            '\'' => enabled = false,
            ')' => enabled = true,
            _ => {
                // this time we'll regex out the numbers, for something different
                let [a, b] = re2.captures(m).expect("captures").extract().1;
                let [a, b]: [ usize; 2 ] = [ a.parse().expect("number"), b.parse().expect("number") ];

                let r = a * b;
                println!("{enabled:5} {a:3} * {b:3} = {r:6}");
                if enabled {
                    sum2 += r;
                }
            }
        }        
    });
    println!("part one sum: {sum}");
    println!("part two sum: {sum2}");
}

fn day04(input: &String) {
    // word search! for XMAS
    // get text as a grid of chars
    let rows = input.lines().collect::<Vec<&str>>();
    let data: Vec<Vec<char>> = rows.iter().map(|r| r.chars().collect::<Vec::<char>>()).collect();

    let h = rows.len();
    let w = rows[0].len();
    println!("w: {w} h: {h}");
    let mut c = 0;

    const XMAS: [char; 4] = [ 'X', 'M', 'A', 'S' ];
    const SAMX: [char; 4] = [ 'S', 'A', 'M', 'X' ];

    for y in 0..h {
        for x in 0..w {
            // horizontal search
            if x < w-3 {
                let window = &data[y][x..x+4];
                c += (window == XMAS) as usize;
                c += (window == SAMX) as usize;
            }
            // vertical search
            if y < h-3 {
                let window = [ data[y][x], data[y+1][x], data[y+2][x], data[y+3][x] ];
                c += (window == XMAS) as usize;
                c += (window == SAMX) as usize;
            }
            // diagonal TL--BR search
            if x < w-3 && y < h-3 {
                let window = [ data[y][x], data[y+1][x+1], data[y+2][x+2], data[y+3][x+3] ];
                c += (window == XMAS) as usize;
                c += (window == SAMX) as usize;
            }
            // diagonal TR--BL search
            if x >= 3 && y < h-3 {
                let window = [ data[y][x], data[y+1][x-1], data[y+2][x-2], data[y+3][x-3] ];
                c += (window == XMAS) as usize;
                c += (window == SAMX) as usize;
            }
        }
    }

    println!("part one count: {c}");

    // part two: X-MAS
    // a..         ..a
    // .A.   and   .A.
    // ..b         b..
    // the pattern is valid for specific values of a and b, either MS or SM
    
    const MS: [char; 2] = ['M','S'];
    const SM: [char; 2] = ['S','M'];

    let mut c2 = 0;

    for y in 1..h-1 {
        for x in 1..w-1 {
            if data[y][x] == 'A' {
                let mut pass_count = 0;
                let window = [ data[y-1][x-1], data[y+1][x+1] ];            
                pass_count += (window == MS || window == SM) as usize;      // test TL-BR
                let window = [ data[y-1][x+1], data[y+1][x-1] ];            // test TR-BL
                pass_count += (window == MS || window == SM) as usize;
                c2 += (pass_count == 2) as usize;       // increment if this X passes both tests
            }
        }
    }

    println!("part two count: {c2}");
}

fn day05(input: &String) {
    // split input into rules and updates

    let split_point = input.find("\n\n").expect("double newline");

    // find returns the byte offset, we are using it as a char offset, luckily the input is ascii :)

    let rules: Vec<(usize,usize)> = input[0..split_point].split_whitespace().map(
        |s| ( s[0..2].parse::<usize>().expect("2 digit number"), s[3..5].parse::<usize>().expect("2 digit number") )
        ).collect();

    let updates: Vec<Vec<usize>> = input[split_point+2..].split_whitespace().map(        
        |u| u.split(',').map(
            |s| s.parse::<usize>().expect("number")
            ).collect()
        ).collect();

    // rules: a|b means a must be before b

    fn invalid_pair(a: usize, b: usize, rules: &Vec::<(usize,usize)>) -> bool {
        // check if this pair is valid according to the rules
        // find matching rules
        let rf: Vec<&(usize,usize)> = rules.iter().filter(|r| (r.0 == a && r.1 == b) || (r.0 == b && r.1 == a)).collect();
        
        // check for any failures
        rf.iter().any( |r| !(r.0 == a && r.1 == b) )
    }

    let mut sum = 0;
    let mut failed_updates: Vec<Vec<usize>> = vec![];   // keep the failed updates for part two
    
    for i in 0..updates.len() {
        let u = &updates[i];

        let mut f = false;
        for j in 0..u.len()-1 {             // check each number pair in the update
            f |= invalid_pair(u[j], u[j+1], &rules);
        }
        if f {
            println!("update invalid: {0:?}", updates[i]);
            failed_updates.push(updates[i].clone());
        } else {
            println!("update valid  : {0:?}", updates[i]);        
            sum += u[u.len()/2];            // find middle pair and add to sum
        }
    }

    println!("part one sum: {sum}");

    // part two... fix and sum only the incorrect updates
    let mut sum2 = 0;
    for u in failed_updates.iter_mut() {
        println!("original: {u:?}");

        // sort with custom rules
        // note it doesn't actually matter which way we sort -- as we are using just the middle value!
        u.sort_by(|a,b| {
            match invalid_pair(*a,*b,&rules) {
                false => std::cmp::Ordering::Less,
                true => std::cmp::Ordering::Greater,                
            }
        });

        println!("sorted  : {u:?}");
        
        // find middle pair and add to sum
        sum2 += u[u.len()/2];
    }

    println!("part two sum: {sum2}");
}
