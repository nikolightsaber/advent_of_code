use std::{
    error::Error,
    future,
    num::ParseIntError,
    str::FromStr,
    sync::{mpsc, Arc},
    thread,
};

use futures::{executor::block_on, future::join_all};

#[derive(Debug)]
struct Map {
    out: u64,
    seed: u64,
    len: u64,
}
impl Map {
    fn inside(&self, seed: u64) -> Option<u64> {
        if self.seed <= seed && seed < self.seed + self.len {
            return Some(self.out + seed - self.seed);
        }
        None
    }
}

#[derive(Debug)]
struct Adaptor {
    maps: Vec<Map>,
}

impl Adaptor {
    fn adapt(&self, seed: u64) -> u64 {
        for map in self.maps.iter() {
            if let Some(out) = map.inside(seed) {
                return out;
            }
        }
        seed
    }
    async fn adapt_async(&self, seed: u64) -> u64 {
        self.adapt(seed)
    }
}

#[derive(Debug)]
struct AdaptorParseError;
impl From<ParseIntError> for AdaptorParseError {
    fn from(_: ParseIntError) -> Self {
        AdaptorParseError
    }
}

impl FromStr for Adaptor {
    type Err = AdaptorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines();
        iter.next().ok_or(AdaptorParseError)?;
        Ok(Adaptor {
            maps: iter
                .map(|l| {
                    let mut map = l.split_whitespace();
                    Ok(Map {
                        out: map.next().ok_or(AdaptorParseError)?.parse()?,
                        seed: map.next().ok_or(AdaptorParseError)?.parse()?,
                        len: map.next().ok_or(AdaptorParseError)?.parse()?,
                    })
                })
                .collect::<Result<_, AdaptorParseError>>()?,
        })
    }
}

// Litterally gets killed
fn multi_test(array: &mut Vec<u64>, adaptor: Arc<Adaptor>) {
    const THREADS: usize = 6;
    let chunk_size = array.len() / THREADS;

    // Create a vector to hold the thread handles
    let mut handles = vec![];

    let (tx, rx) = mpsc::channel();
    // Spawn threads
    for i in 0..THREADS {
        let mut array_chunk = vec![];
        for _ in 0..chunk_size {
            array_chunk.push(array.pop().unwrap());
        }

        println!("spawning {}", i);
        let ad = adaptor.clone();
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            println!("spawned {}", i);
            for value in array_chunk.iter_mut() {
                tx.send(ad.adapt(*value)).unwrap();
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    for val in rx {
        array.push(val);
    }
}

// async not made for low load lot of them
async fn multi_test2(array: &mut Vec<u64>, adaptor: &Adaptor) {
    const THREADS: usize = 10000;

    for i in (0..array.len()).step_by(THREADS) {
        let mut handles = vec![];
        println!("starting");
        for j in i..std::cmp::min(i + THREADS, array.len()) {
            handles.push(adaptor.adapt_async(array[j]))
        }

        for (k, v) in join_all(handles).await.into_iter().enumerate() {
            array[i + k] = v;
        }
        println!("ending");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("official");

    let mut input_iter = input.split("\n\n");
    let seeds_str = input_iter.next().expect("Wrong input");
    let mut seeds1 = seeds_str
        .split_once(": ")
        .map(|(_, seeds)| {
            seeds
                .split_whitespace()
                .map(str::parse::<u64>)
                .collect::<Result<Vec<_>, _>>()
        })
        .expect("Wrong input")
        .expect("Wrong input");

    let mut seeds2 = seeds_str
        .split_once(": ")
        .map(|(_, seeds)| {
            let d = seeds.split_whitespace().flat_map(str::parse::<u64>);
            let d2 = d.clone();
            d.step_by(2)
                .zip(d2.skip(1).step_by(2))
                .map(|(a, b)| a..(a + b))
                .flatten()
                .collect::<Vec<_>>()
        })
        .expect("Wrong input");

    dbg!(seeds2.len());

    for adaptor in input_iter {
        let adaptor: Adaptor = adaptor.parse().expect("Wrong input");
        for seed in seeds1.iter_mut() {
            *seed = adaptor.adapt(*seed);
        }
        block_on(multi_test2(&mut seeds2, &adaptor));
    }
    println!("{}", seeds1.iter().min().unwrap());
    println!("{}", seeds2.iter().min().unwrap());
    Ok(())
}
