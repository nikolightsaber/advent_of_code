use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../inp_off.txt");
    let out = input
        .lines()
        .flat_map(|line| line.split_once(","))
        .map(|(l, r)| (l.split_once("-").unwrap(), r.split_once("-").unwrap()))
        .map(|((l1, l2), (r1, r2))| {
            (
                l1.parse::<usize>().unwrap(),
                l2.parse::<usize>().unwrap(),
                r1.parse::<usize>().unwrap(),
                r2.parse::<usize>().unwrap(),
            )
        })
        .map(|(l1, l2, r1, r2)| {
            if (l1 <= r1 && r2 <= l2) || (r1 <= l1 && l2 <= r2) {
                1
            } else {
                0
            }
        })
        .sum::<usize>();
    println!("ex 1: {:?}", out);

    let out = input
        .lines()
        .flat_map(|line| line.split_once(","))
        .map(|(l, r)| (l.split_once("-").unwrap(), r.split_once("-").unwrap()))
        .map(|((l1, l2), (r1, r2))| {
            (
                l1.parse::<usize>().unwrap(),
                l2.parse::<usize>().unwrap(),
                r1.parse::<usize>().unwrap(),
                r2.parse::<usize>().unwrap(),
            )
        })
        .map(|(l1, l2, r1, r2)| if l1 > r2 || r1 > l2 { 0 } else { 1 })
        .sum::<usize>();
    println!("ex 2: {:?}", out);
    Ok(())
}
