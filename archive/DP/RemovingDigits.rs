#![allow(unused)]
use std::{
    cmp::{max, min},
    collections::HashMap,
    io,
};

fn dp(n: i32) -> i32 {
    let mut dp: Vec<i32> = vec![1; (n + 1) as usize];

    for i in (10..=n) {
        let mut nn = i;
        let mut biggest_digit = u8::MIN;
        while nn != 0 {
            let r: u8 = (nn % 10) as u8;
            nn /= 10;
            biggest_digit = max(biggest_digit, r);
            if biggest_digit == 9 {
                break;
            }
        }
        let prev = i - biggest_digit as i32;
        dp[i as usize] = 1 + dp[prev as usize];
    }

    *dp.last().unwrap()
}

fn main() {
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");

    let first_line: Vec<i32> = str
        .split_whitespace()
        .map(|x| x.parse().unwrap_or(0))
        .collect();
    let n = first_line[0];
    let result = dp(n);

    print!("{}", result);
}
