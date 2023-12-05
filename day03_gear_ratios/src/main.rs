use std::error::Error;

#[derive(Debug)]
struct PartNumber {
    val: u32,
    x: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_inp.txt");

    let parsed_input = input
        .lines()
        .map(|l| {
            let mut part_numbers: Vec<PartNumber> = vec![];
            let mut symbols: Vec<usize> = vec![];
            let mut tmp_part: Option<PartNumber> = None;
            for (i, c) in l.chars().enumerate() {
                if c == '.' {
                    if let Some(part) = tmp_part.take() {
                        part_numbers.push(part);
                    }
                } else if let Some(digit) = c.to_digit(10) {
                    tmp_part =
                        Some(
                            tmp_part.map_or(PartNumber { val: digit, x: i }, |part| PartNumber {
                                val: part.val * 10 + digit,
                                x: part.x,
                            }),
                        );
                } else {
                    symbols.push(i);
                }
            }
            (part_numbers, symbols)
        })
        .collect::<Vec<_>>();
    println!("{:?}", parsed_input);
    Ok(())
}
