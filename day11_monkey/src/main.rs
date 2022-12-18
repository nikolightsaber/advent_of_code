use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> usize>,
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        // first line is useless
        lines.next();
        let (_, items) = lines
            .next()
            .ok_or("Expected 5 lines")?
            .split_once(":")
            .ok_or("expected items right of a ':' ")?;
        let items: Vec<usize> = items
            .split(",")
            .flat_map(|item| item.trim().parse::<usize>())
            .collect();

        let (_, operation) = lines
            .next()
            .ok_or("Expected 5 lines")?
            .split_once(":")
            .ok_or("expected opration after ':'")?;
        let (_, operation) = operation
            .split_once("=")
            .ok_or("Expected opration after '=' sign")?;

        let (operation, l, r): (Box<dyn Fn(usize, usize) -> usize>, &str, &str) =
            if let Some((l, r)) = operation.split_once("+") {
                (Box::new(|a: usize, b: usize| a + b), l, r)
            } else if let Some((l, r)) = operation.split_once("*") {
                (Box::new(|a: usize, b: usize| a * b), l, r)
            } else {
                return Err(String::from("Expected + or * operator"));
            };

        if !l.contains("old") {
            return Err(String::from("left hand side of operaton should be 'old'"));
        }

        let operation: Box<dyn Fn(usize) -> usize> = match r.trim().parse::<usize>() {
            Ok(val) => Box::new(move |old| operation(old, val)),
            Err(_) => Box::new(move |old| operation(old, old)),
        };

        let (_, test) = lines
            .next()
            .ok_or("expected 5 lines")?
            .split_once(":")
            .ok_or("Expected test after ':'")?;
        let (_, test) = test.split_once("by").ok_or("Expected 'by' then number")?;
        let div_val: usize = test
            .trim()
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;

        if div_val == 0 {
            return Err(String::from("Div val cannot be null"));
        }

        let (_, test_true) = lines
            .next()
            .ok_or("expected 5 lines")?
            .split_once("monkey")
            .ok_or("Expected test after 'monkey'")?;
        let test_true: usize = test_true
            .trim()
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;

        let (_, test_false) = lines
            .next()
            .ok_or("expected 5 lines")?
            .split_once("monkey")
            .ok_or("Expected test after 'monkey'")?;
        let test_false = test_false
            .trim()
            .parse::<usize>()
            .map_err(|e: ParseIntError| e.to_string())?;

        let test: Box<dyn Fn(usize) -> usize> = Box::new(move |val| {
            if val % div_val == 0 {
                test_true
            } else {
                test_false
            }
        });

        Ok(Monkey {
            items,
            operation,
            test,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    Ok(())
}
