pub fn day01(input: &String) -> (String,String) {
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
    (sum.to_string(), score.to_string())
}