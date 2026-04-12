use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();

    let mut dp = vec![0; k+1];
    dp[0] = 1;
    for i in 1..=n{
        for j in (1..=k).rev(){
            dp[j] += dp[j-1];
        }
    }

    println!("{}", dp[k]);

    
}
