use std::{collections::HashSet, error::Error};

fn pretty_print(map: &Vec<Vec<u8>>, path: &Vec<(usize, usize)>) {
    if path.len() == 0 {
        return;
    }
    for (i, l) in map.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if path.contains(&(i, j)) {
                print!("\x1b[93m{}\x1b[0m", *c as char);
            } else {
                print!("{}", *c as char);
            }
        }
        println!();
    }
    println!("    {}", path.len());
}

fn search(
    map: &Vec<Vec<u8>>,
    last: Vec<(usize, usize)>,
    mut rest: HashSet<(usize, usize)>,
) -> Option<usize> {
    if last.len() == 0 {
        return None;
    }
    let next = last
        .iter()
        .flat_map(|index| {
            [(1, 0), (0, 1), (-1, 0), (0, -1)]
                .into_iter()
                .flat_map(|(i, j): (isize, isize)| {
                    let ni = index.0.checked_add_signed(i)?;
                    let nj = index.1.checked_add_signed(j)?;
                    let height = *map.get(ni)?.get(nj)?;
                    Some((ni, nj, height))
                })
                .filter(|(i, j, height)| {
                    let prev = map[index.0][index.1];
                    !last.contains(&(*i, *j)) && (prev + 1 >= *height) && !rest.contains(&(*i, *j))
                })
        })
        .fold(Some(Vec::new()), |mut opt, n| {
            if map[n.0][n.1] == b'z' + 1 {
                return None;
            }
            if let Some(v) = opt.as_mut() {
                let ind = (n.0, n.1);
                if !v.contains(&ind) {
                    v.push(ind);
                }
            }
            return opt;
        });

    if let Some(wave) = next {
        //pretty_print(map, &wave);
        last.into_iter().for_each(|l| {
            rest.insert(l);
        });
        return search(map, wave, rest).map(|v| v + 1);
    }
    Some(1)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("inp_off.txt")?;
    let mut first = (0, 0);
    let map: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.bytes()
                .enumerate()
                .map(|(j, c)| {
                    if c == b'S' {
                        first = (i, j);
                        return b'a';
                    }
                    if c == b'E' {
                        return b'z' + 1;
                    }
                    c
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let v = vec![first];
    let ex1 = search(&map, v, HashSet::new());
    println!("{:?}", ex1.unwrap());

    let ex2 = map
        .iter()
        .enumerate()
        .flat_map(|(i, v)| {
            v.iter()
                .enumerate()
                .filter(|(_, h)| **h == b'a')
                .flat_map(|(j, _)| search(&map, vec![(i, j)], HashSet::new()))
                .min()
        })
        .min();
    println!("{:?}", ex2.unwrap());

    Ok(())
}
