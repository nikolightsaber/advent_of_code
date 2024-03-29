use std::error::Error;

#[derive(Debug)]
struct PartNumber {
    val: u32,
    x: usize,
}
impl PartNumber {
    fn touches(&self, x: usize) -> bool {
        let min = self.x.saturating_sub(1);
        let max = self.x + self.val_size();
        return min <= x && x <= max;
    }

    fn val_size(&self) -> usize {
        return self.val.to_string().len();
    }
}

#[derive(Debug)]
struct Symbol {
    sym: char,
    x: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("off_inp.txt");

    let (mut part_nrs, symbols) = input
        .lines()
        .map(|l| {
            let mut part_numbers: Vec<PartNumber> = vec![];
            let mut symbols: Vec<Symbol> = vec![];
            let mut tmp_part: Option<PartNumber> = None;
            for (i, c) in l.chars().enumerate() {
                if let Some(digit) = c.to_digit(10) {
                    tmp_part =
                        Some(
                            tmp_part.map_or(PartNumber { val: digit, x: i }, |part| PartNumber {
                                val: part.val * 10 + digit,
                                x: part.x,
                            }),
                        );
                    continue;
                }
                if c != '.' {
                    symbols.push(Symbol { sym: c, x: i });
                }
                if let Some(part) = tmp_part.take() {
                    part_numbers.push(part);
                }
            }
            if let Some(part) = tmp_part.take() {
                part_numbers.push(part);
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
        for symbol in symbols.iter().filter(|s| s.sym == '*') {
            let mut gears = vec![];
            'outer: for y in i.saturating_sub(1)..=std::cmp::min(i + 1, part_nrs.len() - 1) {
                for partnr in part_nrs[y].iter() {
                    if partnr.touches(symbol.x) {
                        gears.push(partnr.val);
                        if gears.len() > 2 {
                            break 'outer;
                        }
                    }
                }
            }
            if gears.len() == 2 {
                out += gears[0] * gears[1];
            }
        }
    }

    println!("part 2 {}", out);

    let mut out = 0;
    for (i, symbols) in symbols.iter().enumerate() {
        for symbol in symbols {
            for y in i.saturating_sub(1)..=std::cmp::min(i + 1, part_nrs.len() - 1) {
                part_nrs[y].retain(|partnr| {
                    if partnr.touches(symbol.x) {
                        out += partnr.val;
                        return false;
                    }
                    true
                });
            }
        }
    }

    println!("part 1 {}", out);

    Ok(())
}
