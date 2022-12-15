use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("inp_test.txt")?;

    let cmds = input
        .lines()
        .flat_map(|line| match line.split_once(" ") {
            Some((_, val)) => vec![0, val.parse::<i32>().expect("value with add")].into_iter(),
            None => vec![0; 1].into_iter(),
        })
        .fold(Vec::<i32>::new(), |mut acc, val| {
            acc.push(
                acc.get((acc.len() as i32 - 1).try_into().unwrap_or(0))
                    // important first value is 1
                    .unwrap_or(&1)
                    + val,
            );
            acc
        });

    cmds.iter()
        .enumerate()
        .for_each(|(i, v)| println!("{}: {}", i, v));
    [20, 60, 100, 140, 180, 220]
        .iter()
        .for_each(|index| println!("{}", cmds[(*index - 1) as usize]));

    let out = [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|index| cmds[(*index - 1) as usize] * index)
        .sum::<i32>();

    print!("ex 1 {}", out);

    Ok(())
}
