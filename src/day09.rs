//use crate::grid::{*};
use itertools::Itertools;
use std::iter;

#[derive(Clone,Copy)]
struct Segment {
    id: Option<usize>,  // None for empty space
    size: usize,        // Size of this segment in blocks
}

impl Segment {
    pub fn to_string(&self) -> String {
        return if self.id.is_some() {
            format!("{0:04};{1:02}", self.id.unwrap(), self.size)
        } else {
            return format!("....;{0:02}", self.size)
        };
    }
}

pub fn day09(input: &String) {
    // first read as numbers
    let input: Vec<usize> = input.trim_end().chars().map(|s| s.to_string().parse::<usize>().expect("number")).collect_vec();

    // each block can hold either a id_number, or free space
    let mut blocks: Vec<Option<usize>> = vec![];
    for i in 0..input.len() {
        let n = input[i];
        let id = i / 2;
        if i % 2 == 0 {
            let mut file_blocks: Vec<Option<usize>> = iter::repeat(Some(id)).take(n).collect_vec();
            blocks.append( &mut file_blocks);
        } else {
            let mut free_blocks: Vec<Option<usize>> = iter::repeat(None).take(n).collect_vec();
            blocks.append(&mut free_blocks);
        }
    }

    // 00...111...2...333.44.5555.6666.777.888899
    println!("disk of {0} blocks", blocks.len());
    let mut free_idx = blocks.iter().position(|&b| b==None);
    let mut last_idx = blocks.len() - 1;
    while free_idx.is_some() && free_idx.unwrap() < last_idx {
        // move block at last_idx to free_idx
        blocks[free_idx.unwrap()] = blocks[last_idx];
        blocks[last_idx] = None;

        // find next block to move
        last_idx -= 1;
        while blocks[last_idx].is_none() {
            last_idx -= 1;
        }

        // find next free spot
        free_idx = blocks.iter().position(|&b| b==None);
    }
    let mut checksum = 0;
    for (i, n) in blocks.iter().enumerate() {
        if n.is_some() {
            checksum += i * n.unwrap();
        }
    }
    println!("part one checksum: {checksum}");

    // part two
    // Attempt to move each file exactly once in order of decreasing file ID number

    // This time we'll store as segments


    // each block can hold either a id_number, or free space
    let mut segs: Vec<Segment> = vec![];
    let mut id: usize = 0;
    for i in 0..input.len() {
        let n = input[i];
        id = i / 2;
        if i % 2 == 0 {
            segs.push(Segment { id: Some(id), size: n });
        } else {
            segs.push(Segment { id: None, size: n });
        }
    }

    // 00...111...2...333.44.5555.6666.777.888899
    println!("max block id: {0}", id);
    loop {
        // find the segment we want to move
        let seg_idx = segs.iter().position(|&s| s.id == Some(id));
        if seg_idx.is_none() {
            break;
        }
        let seg_idx = seg_idx.unwrap();
        let seg = segs.remove(seg_idx);   // remove it! we have to put it back if we can't find free space

        // find free space to move it
        let fs_idx = segs.iter().position(|&s| s.id == None && s.size >= seg.size);
        if fs_idx.is_some() && fs_idx.unwrap() < seg_idx {
            let fs_idx = fs_idx.unwrap();
            let fs = segs[fs_idx].clone();
            // move the file
            let mut replacement: Vec<Segment> = vec![ seg.clone() ];
            // put into position fs_idx, replacing fs with contents of replacement
            segs.splice(fs_idx..fs_idx+1, replacement.into_iter());
            // replace the seg we removed with an empty
            segs.insert(seg_idx, Segment{id:None, size:seg.size});
            // put in empty space if we have any left
            if (fs.size - seg.size) > 0 {
                segs.insert(fs_idx+1, Segment { id: None, size: fs.size - seg.size });
            }
        } else {
            // put it back
            segs.insert(seg_idx, seg);
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
        if id >= 1 {
            id -= 1;
        } else {
            break;
        }
        // print current state
    //     print!("state: ");
    //     for s in &segs {
    //         print!("{0} ", s.to_string());
    //     }
    //     println!();
    }

    // expand the segments into blocks for calculating the checksum
    let blocks: Vec<Option<usize>> = segs.iter().map(
        |s|        // return s.id, size times
        iter::repeat(s.id).take(s.size).collect_vec()
    ).flatten().collect_vec();

    let mut checksum = 0;

    for (i,n) in blocks.iter().enumerate() {
        if n.is_some() {
            checksum += i * n.unwrap();
        }
    }
    println!("part two checksum: {checksum}");
}