use std::io::{self, Read};

const P: i64 = 1234567891;
const R: i64 = 31;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let _L: usize = it.next().unwrap().parse().unwrap();

    let unistring: Vec<char> = it.next().unwrap().chars().collect();

    let mut sum: i64 = 0;

    for (i, &alp) in unistring.iter().enumerate() {
        // sum += ctoi(alp) * R.pow(i as u32) % P;
        sum += (ctoi(alp) * modular_loop(i as u32)) % P;
    }

    println!("{}", sum);
}

fn ctoi(c: char) -> i64 {
    (c as u8 - b'a') as i64 + 1
}
fn modular_loop(exp: u32) -> i64{
    if exp == 0 {
        return 1;
    }
    if exp == 1 {
        return R;
    }
    let mut acc = R;
    for i in 2..=exp {
        acc = (acc * R) % P;
    }
    acc
}

