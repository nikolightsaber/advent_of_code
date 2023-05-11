use std::error::Error;

fn pretty_print(map: &Vec<Vec<u8>>, path: &Vec<(usize, usize)>) {
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
    index: (usize, usize),
    prev_height: u8,
    mut path: &mut Vec<(usize, usize)>,
) -> Option<u8> {
    if path.contains(&index) {
        return None;
    }
    //dbg!(index);
    let height = map.get(index.0)?.get(index.1)?;
    if prev_height != *height && prev_height + 1 != *height && prev_height - 1 != *height {
        return None;
    }
    if *height == b'z' + 1 {
        pretty_print(map, path);
        return Some(0);
    }
    path.push(index);
    [(1, 0), (0, 1), (-1, 0), (0, -1)]
        .into_iter()
        .flat_map(|(i, j): (isize, isize)| {
            Some((
                index.0.checked_add_signed(i)?,
                index.1.checked_add_signed(j)?,
            ))
        })
        .flat_map(|i| search(map, i, *height, &mut path))
        .min()
        .map_or_else(
            || {
                path.pop();
                None
            },
            |v| Some(v + 1),
        )
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

    let mut v = Vec::<(usize, usize)>::new();
    let a = search(&map, first, b'a', &mut v);
    println!("{:?}", a);

    Ok(())
}
