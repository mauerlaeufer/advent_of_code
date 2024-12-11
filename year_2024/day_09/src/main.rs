use std::collections::BTreeSet;
use std::time::Instant;

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();
    let input = input.trim();
    let now = Instant::now();
    println!("Part 1: {}", part_1(input));
    println!("Took {:?}", now.elapsed());

    let now = Instant::now();
    println!("Part 2: {}", part_2(input));
    println!("Took {:?}", now.elapsed());
}

fn part_1(input: &str) -> usize {
    let mod_input: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut drive: Vec<usize> = Vec::with_capacity(input.len());
    for (pos, rep) in mod_input.iter().enumerate() {
        // Magic - think about this for a second.
        let num = (pos / 2 + 1) * ((pos + 1) % 2);
        drive.append(&mut vec![num; *rep])
    }
    let mut l = 0;
    let mut r = drive.len() - 1;
    while l < r {
        if drive[l] == 0 {
            while l < r && drive[r] == 0 {
                r -= 1;
            }
            drive.swap(l, r);
        }
        l += 1;
    }

    drive[0..r]
        .iter()
        .enumerate()
        .map(|tup| tup.0 * (tup.1 - 1))
        .sum::<usize>()
}

fn part_2(input: &str) -> usize {
    let mod_input: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut drive: Vec<usize> = Vec::with_capacity(input.len());
    let mut free_vecs: Vec<Vec<usize>> = (0..10).map(|_| Vec::new()).collect();

    let mut cur_position = 0;
    for (pos, rep) in mod_input.iter().enumerate() {
        // Remember free space
        if pos % 2 == 1 {
            // Case free space
            free_vecs[*rep].push(cur_position);
        }

        // Build drive like in Part 1
        let num = (pos / 2 + 1) * ((pos + 1) % 2);
        drive.append(&mut vec![num; *rep]);

        cur_position += rep;
    }
    // Cheaper to reverse than to push to front.
    let mut free_btrees: Vec<BTreeSet<usize>> = Vec::new();
    for vec in free_vecs {
        free_btrees.push(vec.into_iter().collect());
    }

    for (pos, rep) in mod_input.iter().enumerate().rev() {
        cur_position -= rep;

        if pos % 2 == 0 {
            // Case content.

            let mut current_lowest = usize::MAX;
            let mut first_size = 0;
            for empty_space_size in *rep..10 {
                // Find the btree containing the first element where my current repetition size fits.
                if let Some(empty_pos) = free_btrees[empty_space_size].first() {
                    if *empty_pos > cur_position {
                        // We never need to check this one again - we passed it.
                        free_btrees[empty_space_size] = BTreeSet::new();
                        continue;
                    }

                    if *empty_pos < current_lowest {
                        current_lowest = *empty_pos;
                        first_size = empty_space_size;
                    }
                }
            }

            if first_size == 0 {
                // println!("Couldn't fit!");
            } else {
                let empty_pos = free_btrees[first_size].pop_first().unwrap();

                let rest = first_size - rep;
                for l in 0..*rep {
                    drive.swap(cur_position + l, empty_pos + l);
                }
                if rest > 0 {
                    if (empty_pos + rep) < cur_position {
                        free_btrees[rest].insert(empty_pos + rep);
                    }
                }
            }
        }
    }

    drive
        .iter()
        .enumerate()
        .filter(|tup| *tup.1 != 0)
        .map(|tup| tup.0 * (tup.1 - 1))
        .sum::<usize>()
}
