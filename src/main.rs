mod tests;

use std::cmp::{max, min};
use std::collections::VecDeque;
use proconio::input;
use std::io::{Read, stdin};

fn main() {
    answer028()
}

fn answer028() {
    input! {
        n: usize,
        a: [i32; n],
    }
    println!("{}", calc028(a))
}

fn calc028(height: Vec<i32>) -> i32 {
    let mut costs = Vec::new();
    costs.push(0);
    costs.push((height[1] - height[0]).abs());
    for i in 2..height.len() {
        let a = (height[i] - height[i - 2]).abs() + costs[i - 2];
        let b = (height[i] - height[i - 1]).abs() + costs[i - 1];
        costs.push(min(a, b));
    }
    return costs.pop().unwrap();
}

fn answer027() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let ans: Vec<String> = calc027(VecDeque::from(a)).iter().map(|i| i.to_string()).collect();
    println!("{}", ans.join(" "))
}

fn calc027(mut nums: VecDeque<u32>) -> VecDeque<u32> {
    if nums.len() == 1 {
        return nums;
    }

    let m = nums.len() / 2;
    let right = nums.split_off(m);
    let left = nums;
    let mut a = calc027(left);
    let mut b = calc027(right);
    let mut c = VecDeque::new();

    while a.len() > 0 || b.len() > 0 {
        if a.len() == 0 && b.len() > 0 {
            c.push_back(b.pop_front().unwrap());
        } else if b.len() == 0 && a.len() > 0 {
            c.push_back(a.pop_front().unwrap())
        } else if a[0] <= b[0] {
            c.push_back(a.pop_front().unwrap())
        } else {
            c.push_back(b.pop_front().unwrap());
        }
    }
    return c;
}

fn answer026() {
    input! {
        n: u32,
    }
    println!("{}", calc026(n))
}

fn calc026(n: u32) -> f64 {
    let mut sum: f64 = 0.0;
    for i in 0..n {
        sum += n as f64 / (n - i) as f64;
    }
    return sum;
}

fn answer025() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n]
    }
    println!("{}", calc025(a, b))
}

fn calc025(a: Vec<usize>, b: Vec<usize>) -> f32 {
    let mut sum = 0.0;
    for i in 0..a.len() {
        sum += a[i] as f32 / 3.0 + b[i] as f32 * 2.0 / 3.0
    }
    return sum;
}

fn answer024() {
    input! {
        n: usize,
        pq: [(usize, usize); n],
    }
    println!("{}", calc024(pq))
}

fn calc024(pq: Vec<(usize, usize)>) -> f32 {
    let mut sum = 0.0;
    for i in 0..pq.len() {
        sum += pq[i].1 as f32 / pq[i].0 as f32;
    }
    return sum;
}

fn answer023() {
    input! {
        n: usize,
        b: [f64; n],
        r: [f64; n]
    }
    println!("{}", calc023(b, r))
}

fn calc023(b: Vec<f64>, r: Vec<f64>) -> f64 {
    let bsum: f64 = b.iter().sum();
    let rsum: f64 = r.iter().sum();
    return (bsum + rsum) / (b.len() as f64);
}

fn answer022() {
    input! {
        n: usize,
        a: [usize; n],
    }
    println!("{}", calc022(a))
}

fn calc022(ns: Vec<usize>) -> usize {
    let mut arr: [usize; 100000] = [0; 100000];
    for i in ns {
        arr[i] += 1;
    }

    let mut ans = arr[50000] * (arr[50000] - 1) / 2;
    for i in 1..50000 {
        ans += arr[i] * arr[100000 - i]
    }
    return ans;
}

fn answer021() {
    input! {
        n: i32,
        r: i32
    }
    println!("{}", calc021(n, r))
}

fn calc021(n: i32, r: i32) -> i64 {
    return factorial(n) / factorial(r) / factorial(n - r);
}

fn factorial(n: i32) -> i64 {
    let mut result: i64 = 1;
    for i in 1_i64..=n.into() {
        result *= i
    }
    return result;
}

fn answer020() {
    input! {
        n: usize,
        a: [i64; n],
    }
    println!("{}", calc020(a))
}

fn calc020(nums: Vec<i64>) -> i32 {
    let mut ans = 0;
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            for k in j + 1..nums.len() {
                for l in k + 1..nums.len() {
                    for m in l + 1..nums.len() {
                        if nums[i] + nums[j] + nums[k] + nums[l] + nums[m] == 1000 {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }
    return ans;
}

fn answer019() {
    input! {
        n: usize,
        a: [i64; n],
    }
    println!("{}", calc019(a))
}

fn calc019(nums: Vec<i64>) -> i64 {
    let mut red = 0;
    let mut yellow = 0;
    let mut blue = 0;
    for i in nums {
        match i {
            1 => red += 1,
            2 => yellow += 1,
            3 => blue += 1,
            _ => panic!("invalid"),
        }
    }
    return (red * (red - 1) / 2) + (yellow * (yellow - 1) / 2) + (blue * (blue - 1) / 2);
}

fn answer018() {
    input! {
        n: usize,
        a: [i64; n],
    }
    println!("{}", calc018(a))
}

fn calc018(nums: Vec<i64>) -> i64 {
    let mut counts = vec![0, 0, 0, 0];
    for i in nums {
        match i {
            100 => counts[0] += 1,
            200 => counts[1] += 1,
            300 => counts[2] += 1,
            400 => counts[3] += 1,
            _ => panic!("invalid"),
        }
    }
    return counts[0] * counts[3] + counts[1] * counts[2];
}

fn answer017() {
    let mut count = String::new();
    let mut numsString = String::new();
    std::io::stdin().read_line(&mut count);
    std::io::stdin().read_line(&mut numsString).unwrap();
    let nums = numsString.split_whitespace().map({ |n| n.parse::<i64>().unwrap() }).collect();
    println!("{}", calc017(nums))
}

fn calc017(nums: Vec<i64>) -> i64 {
    let mut n = nums[0];
    for i in 1..nums.len() {
        let d = calc015(n, nums[i]);
        n = n / d * nums[i];
    }
    return n;
}

fn answer016() {
    let mut count = String::new();
    let mut numsString = String::new();
    std::io::stdin().read_line(&mut count);
    std::io::stdin().read_line(&mut numsString).unwrap();
    let nums = numsString.split_whitespace().map({ |n| n.parse::<i64>().unwrap() }).collect();
    println!("{}", calc016(nums))
}

fn calc016(nums: Vec<i64>) -> i64 {
    let mut n = nums[0];
    for i in 1..nums.len() {
        n = calc015(n, nums[i]);
    }
    return n;
}

fn answer015() {
    input! {
        a: i64,
        b: i64,
    }
    println!("{}", calc015(a, b))
}

fn calc015(a: i64, b: i64) -> i64 {
    let mut x = a;
    let mut y = b;
    while x > 0 && y > 0 {
        if x > y {
            x = x % y
        } else {
            y = y % x
        }
    }
    return max(x, y);
}

fn answer014() {
    input! {
        n: i64,
    }
    let result: Vec<String> = calc014(n).iter().map(|i| i.to_string()).collect();
    println!("{}", result.join(" "));
}

fn calc014(n: i64) -> Vec<i64> {
    let mut vec = Vec::new();
    let rootn = (n as f64).sqrt().round() as i64;
    let mut s = n;
    let mut i = 2;
    while i <= rootn {
        if s % i == 0 {
            vec.push(i);
            s = s / i;
        } else {
            i += 1;
        }
    }
    if s > 1 {
        vec.push(s);
    }
    return vec;
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