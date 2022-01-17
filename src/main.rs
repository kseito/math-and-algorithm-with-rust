mod tests;

use proconio::input;
use std::io::{Read, stdin};

fn main() {
    answer003()
}

fn answer003() {
    let mut count = String::new();
    let mut nums = String::new();
    std::io::stdin().read_line(&mut count);
    std::io::stdin().read_line(&mut nums).unwrap();
    println!("{}", nums.split_whitespace().map({|n| n.parse::<i32>().unwrap()}).sum::<i32>());
}

fn calc003(numbers: Vec<i32>) -> i32 {
    return numbers.iter().sum()
}

fn answer002() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }
    println!("{}", calc002(a, b, c));
}

fn calc002(a: i32, b: i32, c: i32) -> i32 {
    return a + b + c;
}

fn answer001() {
    input! {
        input: i32,
    }
    println!("{}", calc001(input));
}

fn calc001(n: i32) -> i32 {
    return 5 + n;
}