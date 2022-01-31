mod tests;

use proconio::input;
use std::io::{Read, stdin};
use std::iter::repeat;

fn main() {
    answer013()
}

fn answer013() {
    input! {
        n: i64,
    }
    let result = calc013(n);
    result.iter().for_each(|i| println!("{}", i));
}

fn calc013(n: i64) -> Vec<i64> {
    let mut list = Vec::new();
    let rootn = (n as f64).sqrt().round() as i64;
    for i in 1..=rootn {
        let s = n % i;
        if s == 0 {
            list.push(i);
            list.push(n / i)
        }
    }
    return list;
}

fn answer012() {
    input! {
        n: i64,
    }
    if calc012(n) {
        println!("Yes")
    } else {
        println!("No")
    }
}

fn calc012(n: i64) -> bool {
    let rootn = (n as f64).sqrt().round() as i64;
    for i in 2..rootn {
        let s = n % i;
        if s == 0 {
            return false;
        }
    }
    return true;
}

fn answer011() {
    input! {
        n: i32,
    }
    let result: Vec<String> = calc011(n).iter().map(|i| i.to_string()).collect();
    println!("{}", result.join(" "));
}

fn calc011(n: i32) -> Vec<i32> {
    let mut ans = Vec::new();
    for i in 2..=n {
        if isPrime(i) == true {
            ans.push(i)
        }
    }
    return ans;
}

fn isPrime(n: i32) -> bool {
    for i in 2..n {
        let amari = n % i;
        if amari == 0 {
            return false;
        }
    }
    return true;
}

fn answer010() {
    input! {
        n: i64,
    }
    println!("{}", calc010(n));
}

fn calc010(n: i64) -> i64 {
    let mut factorial: i64 = 1;
    for i in 1_i64..=n {
        factorial *= i
    }
    return factorial;
}

// cannot pass all test cases
fn answer009() {
    let mut input = String::new();
    let mut nsString = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    std::io::stdin().read_line(&mut nsString);
    let mut iterator = input.split_whitespace();
    let n: i32 = iterator.next().unwrap().parse().unwrap();
    let s: i32 = iterator.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = nsString.split_whitespace().map({ |n| n.parse::<i32>().unwrap() }).collect();
    if calc009(nums, s) {
        println!("Yes")
    } else {
        println!("No")
    }
}

fn calc009(ns: Vec<i32>, s: i32) -> bool {
    for i in 0..2_i32.pow(ns.len() as u32) {
        let mut sum = 0;
        for j in 1..=ns.len() {
            let digit = j as u32 - 1;
            if i & 2_i32.pow(digit) != 0 {
                sum += ns[j - 1];
            }
        }
        if sum == s {
            return true;
        }
    }
    return false;
}

fn answer008() {
    input! {
        n: i32,
        s: i32,
    }
    println!("{}", calc008(n, s));
}

fn calc008(n: i32, s: i32) -> i32 {
    let mut count = 0;
    for i in 1..=n {
        for j in 1..=n {
            if i + j <= s {
                count += 1;
            }
        }
    }
    return count;
}

fn answer007() {
    input! {
        n: i32,
        x: i32,
        y: i32,
    }
    println!("{}", calc007(n, x, y));
}

fn calc007(n: i32, x: i32, y: i32) -> usize {
    let mut count = 0;
    for i in 1..=n {
        if i % x == 0 || i % y == 0 {
            count += 1
        }
    }
    return count;
}

fn answer006() {
    input! {
        n: i32,
    }
    println!("{}", calc006(n))
}

fn calc006(n: i32) -> i32 {
    return n * 2 + 3;
}

fn answer005() {
    let mut count = String::new();
    let mut numsString = String::new();
    std::io::stdin().read_line(&mut count);
    std::io::stdin().read_line(&mut numsString).unwrap();
    let nums = numsString.split_whitespace().map({ |n| n.parse::<i32>().unwrap() }).collect();
    println!("{}", calc005(nums))
}

fn calc005(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().sum();
    return sum % 100;
}

fn answer004() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }
    println!("{}", calc004(a, b, c));
}

fn calc004(a: i32, b: i32, c: i32) -> i32 {
    return a * b * c;
}

fn answer003() {
    let mut count = String::new();
    let mut nums = String::new();
    std::io::stdin().read_line(&mut count);
    std::io::stdin().read_line(&mut nums).unwrap();
    println!("{}", nums.split_whitespace().map({ |n| n.parse::<i32>().unwrap() }).sum::<i32>());
}

fn calc003(numbers: Vec<i32>) -> i32 {
    return numbers.iter().sum();
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