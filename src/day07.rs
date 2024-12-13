pub fn day07(input: &String) -> (usize,usize) {
    // missing operators puzzle

    let rows = input.lines().collect::<Vec<&str>>();
    let data: Vec<Vec<usize>> = rows.iter().map(|r| {
        let mut nums: Vec<String> = r.split_whitespace().map(|s| s.to_string()).collect();  // split by whitespace
        nums[0] = nums[0][0..nums[0].len()-1].to_string();                                  // remove colon
        return nums.iter().map(|ns| ns.parse::<usize>().expect("number")).collect();        // convert to numbers
    }).collect();

    // evaluate the equation left to right, operators are + or *
    // we want to know which lines could be valid, and sum those totals
    let valid_data: Vec<Vec<usize>> = data.clone().into_iter().filter(|r| {
        // n.b. the target result is stored in r[0]

        print!("line: {r:?} ");

        let num_ops = r.len() - 2;

        // we will use a bitmap to store what operation we are performing. i.e. 0 for add, 1 for multiply.
        // we will be done when we increment the bitmap and hit op_map_done
        let op_map_done = 1 << (num_ops + 1);

        let mut valid_line = false;

        for op_map in 0..op_map_done {
            // apply the ops specified in op_map to the numbers
            let mut result = r[1];  // start with the leftmost number
            for i in 2..r.len() {
                let op = (op_map >> (i-2)) & 0x01;  // find what operation to perform
                match op {
                    0 => result += r[i],
                    1 => result *= r[i],
                    _ => panic!("invalid op"),
                }
            }
            // does our result match the target result?
            if result == r[0] {
                valid_line = true;
            }
        }
        println!("{valid_line}");
        valid_line
    }).collect();

    let valid_sum1: usize = valid_data.iter().map(|r| r[0]).sum();
    println!("part one sum: {valid_sum1}");

    // part two
    let valid_data: Vec<Vec<usize>> = data.into_iter().filter(|r| {
        // target is in r[0]
        print!("line: {r:?} ");

        const OP_ADD: u8 = 0;
        const _OP_MUL: u8 = 1;
        const OP_JOIN: u8 = 2;
        const OP_OVERFLOW: u8 = 3;

        let num_ops = r.len() - 2;                          // number of operation slots
        let mut ops_map: Vec<u8> = vec![OP_ADD; num_ops];   // where we store what each operation slot is doing

        // fn to increment with carry on the ops map
        fn inc_ops_map(ops_map: &mut Vec<u8>) -> bool {     // return true if we aren't finished yet
            if ops_map.iter().all(|&o| o == OP_JOIN) {
                return false;                               // all combinations exhausted
            }
            let mut carry = true;  // carry in a value to increment the first slot
            for i in 0..ops_map.len() {
                ops_map[i] += carry as u8;              // increment if we have a carry
                carry = ops_map[i] == OP_OVERFLOW;      // do we now have a carry
                if carry {
                    ops_map[i] = OP_ADD;                // reset this slot
                }
            }
            if carry {
                panic!("unexpected carry overflow");
            }
            true
        }

        let mut valid_line = false;
        let mut done = false;

        while !(done || valid_line) {
            // now apply the ops specified in ops_map to the numbers
            let mut result = r[1];
            for i in 2..r.len() {       // for each number
                let op = ops_map[i-2];
                match op {              // perform the operation and store into the accumulator (result)
                    0 => result += r[i],
                    1 => result *= r[i],
                    2 => {
                        let a = result.to_string();
                        let b = r[i].to_string();
                        result = (a + &b).parse().expect("number");
                    },
                    _ => panic!("invalid op"),
                }
            }
            // does result match the target number?
            if result == r[0] {
                valid_line = true;
            }
            done = !inc_ops_map(&mut ops_map);  // increment with carry on ops_map, and set the done flag if we've tried every combo
        }
        println!("{valid_line}");
        valid_line
    }).collect();

    let valid_sum2: usize = valid_data.iter().map(|r| r[0]).sum();
    println!("part two sum: {valid_sum2}");

    (valid_sum1, valid_sum2)
}