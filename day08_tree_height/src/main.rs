use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../inp_off.txt");

    let values = input
        .lines()
        .map(|line| line.as_bytes().into_iter().map(|v| v - b'0'));

    let size = values.clone().count();

    let mut col_max: Vec<i16> = vec![-1i16; size];
    let hash = values.clone().into_iter().enumerate().fold(
        HashMap::<(usize, usize), u8>::new(),
        |mut hash, (row_i, row)| {
            let mut row_max = -1i16;
            for (col_i, val) in row.enumerate() {
                if (val as i16) > row_max {
                    row_max = val as i16;
                    hash.entry((row_i, col_i)).or_insert(val);
                }

                if (val as i16) > col_max[col_i] {
                    col_max[col_i] = val as i16;
                    hash.entry((row_i, col_i)).or_insert(val);
                }
            }
            hash
        },
    );

    let mut col_max: Vec<i16> = vec![-1i16; size];
    let hash =
        values
            .clone()
            .into_iter()
            .rev()
            .enumerate()
            .fold(hash, |mut hash, (mut row_i, row)| {
                row_i = size - row_i - 1;
                let mut row_max = -1i16;
                for (col_i, val) in row.enumerate().rev() {
                    if (val as i16) > row_max {
                        row_max = val as i16;
                        hash.entry((row_i, col_i)).or_insert(val);
                    }

                    if (val as i16) > col_max[col_i] {
                        col_max[col_i] = val as i16;
                        hash.entry((row_i, col_i)).or_insert(val);
                    }
                }
                hash
            });
    println!("ex 1 {:?}", hash.len());

    let grid = &values
        .clone()
        .map(|row| row.collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let score = values
        .clone()
        .into_iter()
        .enumerate()
        .map(|(row_i, row)| {
            row.enumerate()
                .map(move |(col_i, val)| {
                    if col_i == 0 || row_i == 0 || col_i == size - 1 || row_i == size - 1 {
                        return 0;
                    }
                    let mut score = 1;
                    let mut count = 0;
                    for r in (0..row_i).rev() {
                        count += 1;
                        if grid[r][col_i] >= val {
                            break;
                        }
                    }
                    score *= count;
                    count = 0;
                    for r in (row_i + 1)..size {
                        count += 1;
                        if grid[r][col_i] >= val {
                            break;
                        }
                    }
                    score *= count;
                    count = 0;
                    for c in (0..col_i).rev() {
                        count += 1;
                        if grid[row_i][c] >= val {
                            break;
                        }
                    }
                    score *= count;
                    count = 0;
                    for c in (col_i + 1)..size {
                        count += 1;
                        if grid[row_i][c] >= val {
                            break;
                        }
                    }
                    score *= count;
                    return score;
                })
                .max()
        })
        .max();

    println!("ex 2 {:?}", score);

    Ok(())
}
