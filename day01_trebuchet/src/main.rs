use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("off_inp.txt");
    println!(
        "{:?}",
        input
            .lines()
            .map(|s| s.chars().filter(|c| c.is_digit(10)))
            .map(|mut l| {
                let first = l.next()?;
                Some(first.to_digit(10)? * 10 + l.last().unwrap_or(first).to_digit(10)?)
            })
            .sum::<Option<u32>>()
    );

    let input = include_str!("off_inp.txt");
    let lookup_digits = [
        ("one", 1u32),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    println!(
        "{:?}",
        input
            .lines()
            .map(|s| {
                let mut tmp: String = String::new();
                let first = s
                    .chars()
                    .find_map(|c| {
                        tmp.push(c);
                        if let Some(lookup_digit) = lookup_digits.iter().find_map(|(k, v)| {
                            if tmp.contains(k) {
                                Some(*v)
                            } else {
                                None
                            }
                        }) {
                            return Some(lookup_digit);
                        }
                        return c.to_digit(10);
                    })
                    .unwrap();
                tmp.clear();
                let last = s
                    .chars()
                    .rev()
                    .find_map(|c| {
                        tmp.insert(0, c);
                        if let Some(lookup_digit) = lookup_digits.iter().find_map(|(k, v)| {
                            if tmp.contains(k) {
                                Some(*v)
                            } else {
                                None
                            }
                        }) {
                            return Some(lookup_digit);
                        }
                        return c.to_digit(10);
                    })
                    .unwrap();
                return first * 10 + last;
            })
            .sum::<u32>()
    );
    Ok(())
}
