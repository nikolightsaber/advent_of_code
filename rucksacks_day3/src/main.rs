use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../inp_off.txt");
    let out = input
        .lines()
        .map(|line| line.as_bytes().split_at(line.len() / 2))
        .map(|(l, r)| {
            for v in l {
                if r.contains(v) {
                    return v;
                }
            }
            unreachable!("should always contain one");
        })
        .map(|val| {
            if *val > 96u8 {
                *val as isize - 96
            } else {
                *val as isize - 64 + 26
            }
        })
        .sum::<isize>();
    println!("ex 1: {}", out);

    let out = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<&[u8]>>()
        .chunks(3)
        .map(|x| {
            let (p1, p2, p3) = (x[0], x[1], x[2]);
            for v in p1 {
                if p2.contains(v) && p3.contains(v) {
                    return v;
                }
            }
            unreachable!("should always contain one");
        })
        .map(|val| {
            if *val > 96u8 {
                *val as isize - 96
            } else {
                *val as isize - 64 + 26
            }
        })
        .sum::<isize>();
    println!("ex 2: {:?}", out);
    Ok(())
}
