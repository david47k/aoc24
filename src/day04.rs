pub fn day04(input: &String) {
    // word search! for XMAS
    // get text as a grid of chars
    let rows = input.lines().collect::<Vec<&str>>();
    let data: Vec<Vec<char>> = rows.iter().map(|r| r.chars().collect::<Vec<char>>()).collect();

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