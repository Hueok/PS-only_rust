use std::io::{self, BufReader, BufRead, BufWriter, Write};
use std::collections::HashMap;

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());

    let mut n = String::new();
    reader.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    let mut input = String::new();

    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
    }
}
