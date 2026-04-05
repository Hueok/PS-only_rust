use std::io::{self, Read};

const P: usize = 1234567891;
const R: usize = 31;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let _L: usize = it.next().unwrap().parse().unwrap();

    let unistring: Vec<char> = it.next().unwrap().chars().collect();

    let mut sum = 0;

    for (i, &alp) in unistring.iter().enumerate() {
        sum += ctoi(alp) * R.pow(i as u32) % P;
    }

    println!("{}", sum);
}

fn ctoi(c: char) -> usize {
    (c as u8 - b'a') as usize + 1
}
