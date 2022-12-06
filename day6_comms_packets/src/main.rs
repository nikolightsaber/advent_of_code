use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../inp.txt");

    let temp = input.lines().map(|line| line.bytes());
    for line in temp {
        let mut matching: Vec<u8> = Vec::new();
        let mut out = 0;
        for (i, b) in line.enumerate() {
            if matching.len() == 4 {
                if matching.iter().all(|c| *c != b) {
                    dbg!(matching);
                    out = i;
                    break;
                }
                matching.truncate(matching.len() - 2);
            }
            matching.push(b);
        }
        println!("ex1 {}", out)
    }
    Ok(())
}
