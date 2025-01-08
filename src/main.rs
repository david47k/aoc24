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
    let test2 = args.len() == 2 && args[1] == "test2";
    let mut tests_passed: usize = 0;

    if test {
        runsheet.append(&mut gen_test_data());
    } else if test2 {
        runsheet.append(&mut gen_test_data_2());
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
        if test || test2 {
            if result == *eresult {
                tests_passed += 1;
                println!("\n=== DAY {:2} TEST PASSED ===", day);
                println!("part 1: {:?}", result.0);
                println!("part 2: {:?}", result.1);
                println!("==========================");
            } else {
                println!("\n=== DAY {:2} TEST FAILED ===", day);
                println!("expected: {:?}", *eresult);
                println!("got:      {:?}", result);
                println!("==========================");
            }
        } else {
            println!("\n=== DAY {:2} RESULTS ===", day);
            println!("part 1: {:?}", result.0);
            println!("part 2: {:?}", result.1);
            println!("==========================");
        }
    }
    if test || test2 {
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
        ( 18,   "ex18.txt",  ("146", "no solution found") ),
        ( 19,   "ex19.txt",  ("6", "16") ),
        ( 20,   "ex20.txt",  ("0", "0") ),  // not a great test
        ( 21,   "ex21.txt",  ("126384", "154115708116294") ),
        ( 22,   "ex22.txt",  ("37327623", "24") ),
        ( 23,   "ex23.txt",  ("7", "co,de,ka,ta") ),
        ( 24,   "ex24.txt",  ("2024", "unknown") ), // not a great test
        ( 25,   "ex25.txt",  ("3", "no result") ),
    ];
    d.into_iter().map(|(day,fname,(result1,result2))| (day,fname.to_string(),(result1.to_string(), result2.to_string()))).collect_vec()
}

fn gen_test_data_2() -> Vec<(usize,String,(String, String))> {
    let d = vec![
        ( 1,    "input01.txt",   ("1223326", "21070419") ),
        ( 2,    "input02.txt",   ("559", "601") ),
        ( 3,    "input03.txt",   ("167650499", "95846796") ),
        ( 4,    "input04.txt",   ("2613", "1905") ),
        ( 5,    "input05.txt",   ("5208", "6732") ),
        ( 6,    "input06.txt",   ("4789", "1304") ),
        ( 7,    "input07.txt",   ("2664460013123", "426214131924213") ),
        ( 8,    "input08.txt",   ("336", "1131") ),
        ( 9,    "input09.txt",   ("6430446922192", "6460170593016") ),
        ( 10,   "input10.txt",   ("694", "1497") ),
        ( 11,   "input11.txt",   ("233875", "277444936413293") ),
        ( 12,   "input12.txt",   ("1450816", "865662") ),
        ( 13,   "input13.txt",   ("37686", "77204516023437") ),
        ( 14,   "input14.txt",   ("219512160", "6398") ),
        ( 15,   "input15.txt",   ("1552463", "1554058") ),
        ( 16,   "input16.txt",   ("123540", "665") ),
        ( 17,   "input17.txt",   ("1,2,3,1,3,2,5,3,1", "105706277661082") ),
        ( 18,   "input18.txt",   ("436", "61,50") ),
        ( 19,   "input19.txt",   ("263", "723524534506343") ),
        ( 20,   "input20.txt",   ("1372", "979014") ),
        ( 21,   "input21.txt",   ("155252", "195664513288128") ),
        ( 22,   "input22.txt",   ("13461553007", "1499") ),
        ( 23,   "input23.txt",   ("1098", "ar,ep,ih,ju,jx,le,ol,pk,pm,pp,xf,yu,zg") ),
        ( 24,   "input24.txt",   ("55544677167336", "gsd,kth,qnf,tbt,vpm,z12,z26,z32") ), // not a great test
        ( 25,   "input25.txt",   ("3255", "no result") ),
    ];
    d.into_iter().map(|(day,fname,(result1,result2))| (day,fname.to_string(),(result1.to_string(), result2.to_string()))).collect_vec()
}