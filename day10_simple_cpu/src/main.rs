use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("inp_off.txt")?;

    let mut cmds = input
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

    // change applied after execution
    cmds.insert(0, 1);

    // -1 because 0 vs 1 indexed
    let out = [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|index| cmds[(*index - 1) as usize] * index)
        .sum::<i32>();

    println!("ex 1 {} \n\n", out);

    println!("ex 2: \n\n");

    cmds.chunks(40)
        .map(|line| {
            line.iter()
                .enumerate()
                .fold(String::new(), |mut acc, (i, val)| {
                    if (val - (i as i32)).abs() < 2 {
                        acc.push('#');
                    } else {
                        acc.push(' ');
                    }
                    acc
                })
        })
        .for_each(|line| println!("{}", line));

    Ok(())
}
