use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../inp_off.txt");

    let values = input
        .lines()
        .map(|line| line.as_bytes().into_iter().map(|v| v - b'0'));

    let size = values.clone().count();

    let mut col_max: Vec<i16> = vec![-1i16; size];
    let hash = values.clone().enumerate().fold(
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
    let hash = values
        .clone()
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

    Ok(())
}
