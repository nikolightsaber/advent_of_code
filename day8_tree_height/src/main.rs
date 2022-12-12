use std::error::Error;

fn adapt_forw(info: &mut (u8, usize), val: u8, already_added: bool) -> bool {
    if info.0 < val {
        info.0 = val;
        if !already_added {
            info.1 += 1;
            return true;
        }
    }
    return false;
}

fn adapt_backw(info: &mut (u8, usize), val: u8, already_added: bool) -> bool {
    if info.0 < val {
        info.0 = val;
        info.1 = 0;
    }
    if !already_added {
        info.1 += 1;
        return true;
    }

    return false;
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../inp_test.txt");

    let values: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .into_iter()
                .map(|v| v - b'0')
                .collect::<Vec<u8>>()
        })
        .collect();

    let mut top: Vec<(u8, usize)> = vec![(0, 0); values[0].len()];
    let mut bot: Vec<(u8, usize)> = vec![(0, 0); values[0].len()];
    let mut count = 0;

    for line in values {
        let mut left: (u8, usize) = (0, 0);
        let mut right: (u8, usize) = (0, 0);
        dbg!(&line);
        for (i, v) in line.iter().enumerate() {
            let added = adapt_forw(&mut left, *v, false);
            let added = adapt_forw(&mut top[i], *v, added);
            let added = adapt_backw(&mut right, *v, added);
            adapt_backw(&mut bot[i], *v, added);
        }
        dbg!(left, right);
        count += left.1;
        count += right.1;
    }

    dbg!(&top, &bot);
    count = top.iter().fold(count, |count, v| count + v.1);
    count = bot.iter().fold(count, |count, v| count + v.1);

    println!("ex 1 {}", count);

    Ok(())
}
