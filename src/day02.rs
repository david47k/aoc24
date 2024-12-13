pub fn day02(input: &String) -> (usize,usize) {
    // split input by whitespace, and convert to unsigned integers
    let reports = input.split('\n').collect::<Vec<&str>>();
    let reports: Vec<Vec<usize>> = reports.into_iter().map(|r| r.split_whitespace().collect::<Vec<&str>>().into_iter().map(|s| s.parse::<usize>().expect("Input should be unsigned integers")).collect()).collect();

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

    (safe, safe2)
}