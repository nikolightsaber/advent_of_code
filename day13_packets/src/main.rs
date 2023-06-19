use std::{cmp::Ordering, error::Error};

enum PacketUpdateError {
    IsEmpty,
}

#[derive(Debug)]
struct PacketState {
    dept: usize,
    value: Option<usize>,
}

impl PacketState {
    fn new() -> Self {
        PacketState {
            dept: 0,
            value: None,
        }
    }

    fn update(&mut self, ch: Option<char>) -> Result<(), PacketUpdateError> {
        match ch {
            Some('[') => {
                self.value = None;
                self.dept += 1
            }
            Some(']') => {
                self.value = None;
                self.dept -= 1
            }
            Some('1'..='9') => self.value = ch.unwrap().to_digit(10).map(|v| v as usize),
            None => return Err(PacketUpdateError::IsEmpty),
            _ => self.value = None,
        }
        Ok(())
    }
}

impl PartialEq for PacketState {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value && self.dept == other.dept;
    }
}

impl PartialOrd for PacketState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.dept == 0 && other.dept > 0 {
            return Some(Ordering::Less);
        }
        if other.dept == 0 && self.dept > 0 {
            return Some(Ordering::Greater);
        }
        if self.dept != other.dept {
            return None;
        }
        return self.value.partial_cmp(&other.value);
    }
}

fn update_left_and_right(
    left_state: &mut PacketState,
    right_state: &mut PacketState,
    left_char: Option<char>,
    right_char: Option<char>,
) -> Result<(), PacketUpdateError> {
    if let (Err(PacketUpdateError::IsEmpty), Err(PacketUpdateError::IsEmpty)) =
        (left_state.update(left_char), right_state.update(right_char))
    {
        return Err(PacketUpdateError::IsEmpty);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("inp_off.txt")?;

    let ex1: usize =
        input
            .split("\n\n")
            .map(str::lines)
            .flat_map(|mut pair| {
                Some((
                    pair.next()?.chars().peekable(),
                    pair.next()?.chars().peekable(),
                ))
            })
            .enumerate()
            .map(|(i, (mut left, mut right))| {
                let mut left_state = PacketState::new();
                let mut right_state = PacketState::new();
                loop {
                    dbg!(i, &left_state, &right_state);
                    match left_state.partial_cmp(&right_state) {
                        Some(Ordering::Greater) => {
                            return None;
                        }
                        Some(Ordering::Equal) | Some(Ordering::Less) => {
                            if let (
                                Err(PacketUpdateError::IsEmpty),
                                Err(PacketUpdateError::IsEmpty),
                            ) = (
                                left_state.update(left.next()),
                                right_state.update(right.next()),
                            ) {
                                break;
                            }
                        }
                        None => match (left_state.value, right_state.value) {
                            (None, Some(_)) => {
                                let _ = left_state.update(left.next());
                            }
                            (Some(_), None) => {
                                let _ = right_state.update(right.next());
                            }
                            (None, None) => {
                                if let (
                                    Err(PacketUpdateError::IsEmpty),
                                    Err(PacketUpdateError::IsEmpty),
                                ) = (
                                    left_state.update(left.next()),
                                    right_state.update(right.next()),
                                ) {
                                    break;
                                }
                            }
                            (Some(l), Some(r)) if l <= r => {
                                while left_state.dept != right_state.dept {
                                    let _ = left_state.update(left.next());
                                }
                                if let (
                                    Err(PacketUpdateError::IsEmpty),
                                    Err(PacketUpdateError::IsEmpty),
                                ) = (
                                    left_state.update(left.next()),
                                    right_state.update(right.next()),
                                ) {
                                    break;
                                }
                            }
                            (Some(l), Some(r)) if l > r => return None,
                            (Some(_), Some(_)) => unreachable!(),
                        },
                    }
                }
                Some(i + 1)
            })
            .flat_map(|out| dbg!(out))
            .sum();
    println!("ex1: {}", ex1);
    Ok(())
}
