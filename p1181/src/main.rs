use std::io::{self, Read, BufWriter, Write};
use std::cmp::Ordering;

fn main() {
    let mut writer = BufWriter::new(io::stdout().lock());

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();

    let mut words: Vec<&str> = it.collect();
    quicksort(&mut words, 0, n-1);

    for s in words.iter(){
        writeln!(writer, "{}", s);
    }

    
}

fn quicksort(arr: &mut [&str], left: usize, right: usize){
    if left >= right {
        return;
    }
    let pivot_index: usize = partition(arr, left, right);

    if pivot_index > left { quicksort(arr, left, pivot_index-1); }
    if pivot_index < right { quicksort(arr, pivot_index+1, right); }
}

fn partition(arr: &mut [&str], left: usize, right: usize) -> usize {
    let pivot = arr[right];
    let mut i = left;

    for j in left..right {
        if comp(&arr[j], &pivot) != Ordering::Greater {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, right);
    i
}

fn comp(lhs: &str, rhs: &str) -> Ordering {
    lhs.len()
        .cmp(&rhs.len())
        .then(lhs.cmp(rhs))
}

