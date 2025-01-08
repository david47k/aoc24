//use crate::grid::{*};
use itertools::Itertools;
use std::iter;

#[derive(Clone,Copy)]
struct Segment {
    id: Option<usize>,  // None for empty space, Some<file_id> for a file
    size: usize,        // Size of this segment in blocks
}

pub fn day09(input: &String) -> (String,String) {
    // first read as numbers
    let input: Vec<usize> = input.trim_end().chars().map(|s| s.to_string().parse::<usize>().expect("number")).collect_vec();

    // each block can hold either a id_number, or free space
    let mut blocks: Vec<Option<usize>> = vec![];
    for i in 0..input.len() {
        let n = input[i];
        let id = i / 2;
        let mut new_blocks: Vec<Option<usize>>;
        if i % 2 == 0 {
            new_blocks = iter::repeat(Some(id)).take(n).collect_vec();
        } else {
            new_blocks = iter::repeat(None).take(n).collect_vec();
        }
        blocks.append(&mut new_blocks);
    }

    println!("disk size (blocks): {0}", blocks.len());

    println!("part 1 calculating...");

    // find the last block with a file, and the first block that is empty
    let mut last_idx = blocks.len() - 1;
    let mut free_idx = blocks.iter().take(last_idx - 1).position(|&b| b==None);
    while let Some(free_idx_u) = free_idx {
        // move file block to empty block
        blocks[free_idx_u] = blocks[last_idx];
        blocks[last_idx] = None;

        // find next block to move
        last_idx -= 1;
        while blocks[last_idx].is_none() {
            last_idx -= 1;
        }

        // find next free spot
        free_idx = blocks.iter().take(last_idx - 1).position(|&b| b==None);
    }

    let mut checksum = 0;
    for (i, n) in blocks.iter().enumerate() {
        if let Some(n) = n {
            checksum += i * n;
        }
    }
    println!("part one checksum: {checksum}");
    let checksum1 = checksum;

    // part two

    println!("part 2 calculating...");

    // Attempt to move each file exactly once in order of decreasing file ID number
    // This time we'll store as segments
    let mut segs: Vec<Segment> = vec![];
    let mut id: usize = 0;

    // parse the input into segments. each segment can hold either an id_number, or free space.
    for i in 0..input.len() {
        let n = input[i];
        id = i / 2;
        if i % 2 == 0 {
            segs.push(Segment { id: Some(id), size: n });
        } else {
            segs.push(Segment { id: None, size: n });
        }
    }

    println!("max block id: {0}", id);

    // find the segment we want to move
    let mut seg_idx = segs.iter().position(|&s| s.id == Some(id));
    while let Some(seg_i) = seg_idx {   // while we have a segment to move
        let seg = segs[seg_i].clone();

        // find enough free space
        let fs_idx = segs.iter().take(seg_i ).position(|&s| s.id == None && s.size >= seg.size);

        // if we found enough space, move it
        if let Some(fs_i) = fs_idx {
            let fs = segs[fs_i].clone();
            // move the segment into position fs_idx, replacing fs
            segs.splice(fs_i..fs_i+1, vec![ seg.clone() ]);
            // replace the segment we removed with an empty
            segs.splice(seg_i..seg_i+1, vec![ Segment{id:None, size:seg.size} ]);
            // if the file didn't use up all the space, put the free space in segs
            let free_space = fs.size - seg.size;
            if free_space > 0 {
                segs.insert(fs_i+1, Segment { id: None, size: free_space });
            }
        }
        // merge empty segs
        segs = segs.into_iter().coalesce(|prev, curr| {
            if prev.id == None && curr.id == None {
                return Ok(Segment{id:None, size:prev.size+curr.size});
            } else {
                return Err((prev, curr));
            }
        }).collect_vec();
        // move on to next block
        if id < 1 {
            break;
        }
        id -= 1;
        seg_idx = segs.iter().position(|&s| s.id == Some(id));
    }

    // expand the segments into blocks for calculating the checksum
    let blocks: Vec<Option<usize>> = segs.iter().map(
        |s| iter::repeat(s.id).take(s.size).collect_vec()   // return s.id, size times
    ).flatten().collect_vec();

    // calculate part two checksum
    let mut checksum = 0;
    for (i,n) in blocks.iter().enumerate() {
        if let Some(x) = n {
            checksum += i * x;
        }
    }
    println!("part two checksum: {checksum}");

    (checksum1.to_string(), checksum.to_string())
}