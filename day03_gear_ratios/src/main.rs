use std::error::Error;

#[derive(Debug)]
struct PartNumber {
    val: u32,
    x: usize,
}
impl PartNumber {
    fn touches(&self, x: usize) -> bool {
        let min = self.x.saturating_sub(1);
        let max = self.x + self.val_size() + 1;
        return min <= x && x <= max;
    }

    fn val_size(&self) -> usize {
        return self.val.to_string().len();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("off_inp.txt");

    let (mut part_nrs, symbols) = input
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
        .fold((vec![], vec![]), |mut acc, (part_nrs, symbols)| {
            acc.0.push(part_nrs);
            acc.1.push(symbols);
            acc
        });

    let mut out = 0;
    for (i, symbols) in symbols.iter().enumerate() {
        for symbol in symbols {
            for y in i.saturating_sub(1)..=std::cmp::min(i + 1, part_nrs.len() - 1) {
                part_nrs[y].retain(|partnr| {
                    if partnr.touches(*symbol) {
                        out += partnr.val;
                        return false;
                    }
                    true
                });
            }
        }
    }

    println!("{}", out);

    Ok(())
}
