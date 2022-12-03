fn main() {
    let input = include_str!("../inp_ex_off.txt");

    let interm = input.lines().map(|line| {
        line.split(" ")
            .flat_map(|c| c.chars().next())
            .map(|x| x as isize)
    });
    //.flatten()
    let a = interm
        .clone()
        .map(|mut x| {
            let a = x.next().unwrap();
            let b = x.next().unwrap();
            let mut res = (a - b + 25) % 3;
            if res == 1 {
                res = 2;
            } else if res == 2 {
                res = 1;
            }

            b - 87 + res * 3
        })
        .sum::<isize>();

    println!("ex1 {:?}", a);

    let a = interm
        .map(|mut x| {
            let a = x.next().unwrap();
            let b = x.next().unwrap();
            let res = b - 88;
            let mut play = a - 64 + res - 1;
            if play == 4 {
                play = 1;
            } else if play == 0 {
                play = 3;
            }
            res * 3 + play
        })
        .sum::<isize>();

    println!("ex2 {:?}", a)
}
