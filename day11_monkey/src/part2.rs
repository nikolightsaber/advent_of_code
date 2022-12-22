use core::fmt::Debug;
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> usize>,
    activity: usize,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_0 = (self.operation)(0);
        let op_1 = (self.operation)(1);

        let mut t: usize = 1;
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
        items: Vec<usize>,
        operation: Box<dyn Fn(usize) -> usize>,
        test: Box<dyn Fn(usize) -> usize>,
    ) -> Self {
        Monkey {
            items,
            operation,
            test,
            activity: 0,
        }
    }

    // Magic is common divisor of test
    fn cycle_with_magic(&mut self, magic: usize) -> Vec<usize> {
        self.items
            .iter_mut()
            .map(|item| {
                self.activity += 1;
                *item = (self.operation)(*item);
                *item %= magic;
                (self.test)(*item)
            })
            .collect::<Vec<usize>>()
    }

    // A bit dumb but i did not feel like changing the datastructure
    fn get_divisor(&self) -> usize {
        let target = (self.test)(0);
        let mut test = 1;
        while target != (self.test)(test) {
            test += 1;
        }
        test
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

        Ok(Monkey::new(items, operation, test))
    }
}

pub fn solve() -> Result<usize, Box<dyn Error>> {
    let input = std::fs::read_to_string("inp_off.txt")?;

    let mut monkeys = input
        .split("\n\n")
        .flat_map(str::parse::<Monkey>)
        .collect::<Vec<Monkey>>();

    let magic: usize = monkeys.iter().map(|monkey| monkey.get_divisor()).product();
    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            for new in monkeys[i].cycle_with_magic(magic) {
                let item = monkeys[i].items.remove(0);
                monkeys[new].items.push(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.activity.cmp(&a.activity));

    Ok(monkeys[0].activity * monkeys[1].activity)
}
