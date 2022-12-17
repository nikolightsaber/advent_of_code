use std::error::Error;

fn solve(inp: &[u8], win: usize) -> usize {
    inp.windows(win)
        .position(|vals| {
            let mut data: u32 = 0;
            for v in vals {
                let prev = data;
                data |= 1 << (v - b'a');
                if data == prev {
                    return false;
                }
            }
            return true;
        })
        .unwrap()
        + win
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../inp.txt");

    let out = input
        .lines()
        .map(|line| line.as_bytes())
        .map(|set| solve(set, 4))
        .collect::<Vec<usize>>();

    println!("ex1 {:?}", out);

    let out = input
        .lines()
        .map(|line| line.as_bytes())
        .map(|set| solve(set, 14))
        .collect::<Vec<usize>>();

    println!("ex1 {:?}", out);
    Ok(())
}
