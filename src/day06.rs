pub fn day06(input: &String) -> (usize,usize) {
    // where does the guard go?
    // read the input into a vec<vec<char>>
    let rows = input.lines().collect::<Vec<&str>>();
    let mut data: Vec<Vec<char>> = rows.iter().map(|r| r.chars().collect::<Vec<char>>()).collect();

    let h = rows.len();
    let w = rows[0].len();
    println!("w: {w} h: {h}");

    // define directions using a bitmask
    const UP: u8 = 1u8;
    const RIGHT: u8 = 2u8;
    const DOWN: u8 = 4u8;
    const LEFT: u8 = 8u8;

    // where is the guard to start?
    let mut gx = -1;
    let mut gy = -1;
    let mut gd = 0u8;     // guard direction - 0 up, 1 right, 2 down, 3 left
    for y in 0..h {
        for x in 0..w {
            if data[y][x] != '.' && data[y][x] != '#' {
                gx = x as isize;
                gy = y as isize;
                gd = match data[y][x] {
                    '^' => UP,
                    '>' => RIGHT,
                    'v' => DOWN,
                    '<' => LEFT,
                    _ => panic!("cannot recognise guard direction"),
                };
                break;
            }
        }
        if gx != -1 {
            break;
        }
    }
    if gx == -1 {
        panic!("failed to locate guard");
    }
    println!("gx: {gx} gy: {gy}");

    // save guard start location and direction for part two
    let gs = (gx as usize,gy as usize);
    let gds = gd;

    // remove guard from input to make things easier
    data[gy as usize][gx as usize] = '.';

    // vec to keep track of guards position and direction
    let mut visited: Vec<Vec<u8>> = vec![];
    for _ in 0..h {
        let v: Vec<u8> = vec![0; w];
        visited.push(v);
    }
    visited[gy as usize][gx as usize] = gd;

    // given a direction, return a delta (x, y)
    fn map_dir(d: u8) -> (isize, isize) {
        match d {
            UP => (0,-1),
            RIGHT => (1,0),
            DOWN => (0,1),
            LEFT => (-1,0),
            _ => panic!("invalid direction"),
        }
    }

    // return what is in the location, either . (nothing) or # (obstruction) or ! (out of area)
    fn peek(x: isize, y: isize, w: usize, h: usize, data: &Vec<Vec<char>>) -> char {
        if x < 0 || x >= w as isize || y < 0 || y >= h as isize {
            return '!';
        }
        data[y as usize][x as usize]
    }

    // main walking loop
    let mut in_map = true;
    while in_map {
        // peek next location
        let (dx,dy) = map_dir(gd);
        let (nx,ny) = (gx+dx,gy+dy);
        let p = peek(nx, ny, w, h, &data);
        match p {
            '!' => in_map = false,      // out of map
            '#' => {                    // obstruction, turn 90 degrees
                gd <<= 1;
                if gd == 16 {
                    gd = UP;
                }
            },
            '.' => {                    // walk the guard
                gx = nx;                // save guards new position
                gy = ny;
                visited[gy as usize][gx as usize] |= gd;    // store direction guard moved in this position
            },
            _ => panic!("invalid return value from peek"),
        }
    }

    // how many positions the guard visited
    let v = visited.iter().flatten().collect::<Vec<&u8>>().iter().filter(|v| ***v != 0).count();
    println!("part one: {v}");

    // part two
    // place an obstruction on the guards existing path (i.e. in visited above) but not the starting position
    // we want to know if it will form a loop
    // we need to store the guard's DIRECTIONS for each position, too!

    // get a list of (x,y) where we could place an obstruction
    let sites: Vec<(usize, usize)> = visited.iter().enumerate().map(|(y,row)| {
        row.iter().enumerate().map(|(x,d)| {
            if *d != 0 {
                return Some((x,y));
            } else {
                return None;
            }
        }).collect::<Vec<Option<(usize,usize)>>>()
    }).flatten().filter(|d| d.is_some()).map(|d| d.unwrap()).collect();

    println!("sites: {sites:?}");
    println!("sites len: {0:?}", sites.len());

    // remove guard start position
    let sites: Vec<&(usize, usize)> = sites.iter().filter(|&&d| d != gs).collect();

    let mut looped_count = 0;       // number of times the obstruction leads to the guard walking a loop

    // test each (x,y) on a input copy, with the obstruction placed, and check for a guard walk loop
    for (x,y) in sites.iter() {
        print!("trying obstruction at {x:3},{y:3}: ");
        // add an obstruction
        let mut d = data.clone();
        d[*y][*x] = '#';
        // set guard location to start location
        gx = gs.0 as isize;
        gy = gs.1 as isize;
        gd = gds;
        // a 2d vec to keep track of guard positions and directions
        let mut visited2: Vec<Vec<u8>> = vec![];
        for _ in 0..h {
            let v: Vec<u8> = vec![0; w];
            visited2.push(v);
        }
        visited2[gy as usize][gx as usize] = gd;

        // walk the guard
        let mut in_map = true;
        let mut is_loop = false;
        while in_map && !is_loop {
            // peek next location
            let (dx,dy) = map_dir(gd);
            let (nx,ny) = (gx+dx,gy+dy);
            let p = peek(nx, ny, w, h, &d);
            match p {
                '!' => in_map = false,      // guard is off the map
                '#' => {                    // turn 90 degrees
                    gd <<= 1;
                    if gd == 16 {
                        gd = UP;
                    }
                },
                '.' => {                    // walk the guard
                    gx = nx;                // set guards new position
                    gy = ny;
                    if visited2[gy as usize][gx as usize] & gd != 0 {   // have we been here before, in this direction?
                        is_loop = true;
                    } else {
                        visited2[gy as usize][gx as usize] |= gd;       // save the direction we walked to this position
                    }
                },
                _ => panic!("invalid return value from peek"),
            }
        }
        if is_loop {
            println!("looped");
            looped_count += 1;
        } else {
            println!("off map");
        }

    }
    println!("part two looped_count: {looped_count}");

    (v, looped_count)
}