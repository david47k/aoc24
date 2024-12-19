// Advent of Code 2024
// By david47k at d47 dot co

use itertools::Itertools;

pub mod grid;
pub mod vector;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;
pub mod time;
pub mod level;
pub mod defs;
pub mod solve;
pub mod stackstack;
mod path;
mod path2;
mod pathtrait;
mod obj;

fn main() {
    println!("Advent of Code 2024");
    println!("By david47k at d47 dot co");

    let mut runsheet: Vec<(usize,String,(String,String))> = vec![];

    let args: Vec<String> = std::env::args().collect();

    let test = args.len() == 2 && args[1] == "test";
    let mut tests_passed: usize = 0;

    if test {
        runsheet.append(&mut gen_test_data());
    } else {
        if args.len() < 3 {
            println!("Specify which day and input file as arguments (e.g. aoc24 1 ex01.txt)");
            return;
        }
        let day: usize = args[1].parse().expect("valid number for day");
        let fname = args[2].parse().expect("valid input filename");
        runsheet.push((day, fname, ("0".to_string(), "0".to_string())));
    }

    for (day,fname,eresult) in runsheet.iter() {
        println!("\nRunning day {day}:\n");
        let input: String = std::fs::read_to_string(fname).expect("should be able to read file");

        let result: (String, String) = match day {
            1  => day01::day01(&input),
            2  => day02::day02(&input),
            3  => day03::day03(&input),
            4  => day04::day04(&input),
            5  => day05::day05(&input),
            6  => day06::day06(&input),
            7  => day07::day07(&input),
            8  => day08::day08(&input),
            9  => day09::day09(&input),
            10 => day10::day10(&input),
            11 => day11::day11(&input),
            12 => day12::day12(&input),
            13 => day13::day13(&input),
            14 => day14::day14(&input),
            15 => day15::day15(&input),
            16 => day16::day16(&input),
            17 => day17::day17(&input),
            18 => day18::day18(&input),
            19 => day19::day19(&input),
            20 => day20::day20(&input),
            21 => day21::day21(&input),
            22 => day22::day22(&input),
            23 => day23::day23(&input),
            24 => day24::day24(&input),
            25 => day25::day25(&input),
            _  => { println!("Unknown day!"); ("no result".to_string(), "no result".to_string()) }
        };
        if test {
            if result == *eresult {
                tests_passed += 1;
            } else {
                println!("=== TEST FAILED ===");
            }
        }
    }
    if test {
        println!("\nTests conducted : {:2}", runsheet.len());
        println!("Tests failed    : {:2}\n", runsheet.len() - tests_passed);
    }
}

fn gen_test_data() -> Vec<(usize,String,(String, String))> {
    let d = vec![
        ( 1,    "ex01.txt",   ("11", "31") ),
        ( 2,    "ex02.txt",   ("2", "4") ),
        ( 3,    "ex03p2.txt", ("161", "48") ),
        ( 4,    "ex04.txt",   ("18", "9") ),
        ( 5,    "ex05.txt",   ("143", "123") ),
        ( 6,    "ex06.txt",   ("41", "6") ),
        ( 7,    "ex07.txt",   ("3749", "11387") ),
        ( 8,    "ex08.txt",   ("14", "34") ),
        ( 9,    "ex09.txt",   ("1928", "2858") ),
        ( 10,   "ex10.txt",   ("36", "81") ),
        ( 11,   "ex11.txt",   ("55312", "65601038650482") ),
        ( 12,   "ex12c.txt",  ("1930", "1206") ),
        ( 13,   "ex13.txt",   ("480", "875318608908") ),
        ( 14,   "ex14.txt",   ("12", "0") ),
        ( 15,   "ex15.txt",   ("2028", "1751") ),
        ( 15,   "ex15b.txt",  ("10092", "9021") ),
        ( 15,   "ex15c.txt",  ("908", "618") ),
        ( 16,   "ex16.txt",   ("7036", "45") ),
        ( 16,   "ex16b.txt",  ("11048", "64") ),
        ( 17,   "ex17b.txt",  ("5,7,3,0", "117440") ),
    ];
    d.into_iter().map(|(day,fname,(result1,result2))| (day,fname.to_string(),(result1.to_string(), result2.to_string()))).collect_vec()
}