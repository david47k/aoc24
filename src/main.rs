// Advent of Code 2024
// By david47k at d47 dot co

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
        21 => day20::day20(&input),
        22 => day20::day20(&input),
        23 => day20::day20(&input),
        24 => day20::day20(&input),
        25 => day20::day20(&input),
        _  => println!("Unknown day!"),
    };
}
