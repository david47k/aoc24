pub fn day05(input: &String) -> (String,String) {
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

    fn invalid_pair(a: usize, b: usize, rules: &Vec<(usize,usize)>) -> bool {
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
            //println!("update invalid: {0:?}", updates[i]);
            failed_updates.push(updates[i].clone());
        } else {
            //println!("update valid  : {0:?}", updates[i]);
            sum += u[u.len()/2];            // find middle pair and add to sum
        }
    }

    println!("part one sum: {sum}");

    // part two... fix and sum only the incorrect updates
    let mut sum2 = 0;
    for u in failed_updates.iter_mut() {
        //println!("original: {u:?}");

        // sort with custom rules
        // note it doesn't actually matter which way we sort -- as we are using just the middle value!
        u.sort_by(|a,b| {
            match invalid_pair(*a,*b,&rules) {
                false => std::cmp::Ordering::Less,
                true => std::cmp::Ordering::Greater,
            }
        });

        //println!("sorted  : {u:?}");

        // find middle pair and add to sum
        sum2 += u[u.len()/2];
    }

    println!("part two sum: {sum2}");

    (sum.to_string(), sum2.to_string())
}