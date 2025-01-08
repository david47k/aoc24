pub fn day03(input: &String) -> (String,String) {
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
                // println!("{enabled:5} {a:3} * {b:3} = {r:6}");
                if enabled {
                    sum2 += r;
                }
            }
        }
    });
    println!("part one sum: {sum}");
    println!("part two sum: {sum2}");

    (sum.to_string(), sum2.to_string())
}