use core::fmt::Debug;
use std::error::Error;
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

struct Monkey {
    items: Vec<f64>,
    operation: Box<dyn Fn(f64) -> f64>,
    test: Box<dyn Fn(f64) -> usize>,
    activity: usize,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_0 = (self.operation)(0);
        let op_1 = (self.operation)(1);

        let mut t: f64 = 1;
        let test_0 = (self.test)(t);
        while (self.test)(t) == test_0 {
            t += 1;
        }
        let test_1 = (self.test)(t);
        write!(
            f,
            "Monkey ( items: {:?}, operaton: (0 -> {}, 1 -> {}), test: (div: {}, 0: {}, 1: {}), activity: {})",
            self.items, op_0, op_1, t, test_0, test_1, self.activity
        )
    }
}

impl Monkey {
    fn new(
        items: Vec<f64>,
        operation: Box<dyn Fn(f64) -> f64>,
        test: Box<dyn Fn(f64) -> f64>,
    ) -> Self {
        Monkey {
            items,
            operation,
            test,
            activity: 0,
        }
    }

    fn cycle_without_divide(&mut self) -> Vec<f64> {
        self.items
            .iter_mut()
            .map(|item| {
                self.activity += 1;
                *item = (self.operation)(*item);
                (self.test)(*item)
            })
            .collect::<Vec<f64>>()
    }
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
        let items: Vec<f64> = items
            .split(",")
            .flat_map(|item| item.trim().parse::<f64>())
            .collect();

        let (_, operation) = lines
            .next()
            .ok_or("Expected 5 lines")?
            .split_once(":")
            .ok_or("expected opration after ':'")?;
        let (_, operation) = operation
            .split_once("=")
            .ok_or("Expected opration after '=' sign")?;

        let (operation, l, r): (Box<dyn Fn(f64, f64) -> f64>, &str, &str) =
            if let Some((l, r)) = operation.split_once("+") {
                (Box::new(|a: f64, b: f64| a + b), l, r)
            } else if let Some((l, r)) = operation.split_once("*") {
                (Box::new(|a: f64, b: f64| a * b), l, r)
            } else {
                return Err(String::from("Expected + or * operator"));
            };

        if !l.contains("old") {
            return Err(String::from("left hand side of operaton should be 'old'"));
        }

        let operation: Box<dyn Fn(f64) -> f64> = match r.trim().parse::<f64>() {
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

        let test: Box<dyn Fn(f64) -> usize> = Box::new(move |val| {
            if (val as usize) % div_val == 0 {
                test_true
            } else {
                test_false
            }
        });

        Ok(Monkey::new(items, operation, test))
    }
}

pub fn solve() -> Result<f64, Box<dyn Error>> {
    let input = std::fs::read_to_string("inp_test.txt")?;

    let mut monkeys = input
        .split("\n\n")
        .flat_map(str::parse::<Monkey>)
        .collect::<Vec<Monkey>>();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for new in monkeys[i].cycle_without_divide() {
                let item = monkeys[i].items.remove(0);
                monkeys[new].items.push(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.activity.cmp(&a.activity));

    Ok(monkeys[0].activity * monkeys[1].activity)
}
