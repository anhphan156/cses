#![allow(unused)]
use std::{cmp::min, collections::HashMap, io};

fn coin_iter(money: i32, num_coins: i32, coins: &Vec<i32>) -> u64 {
    const MOD: u64 = u64::pow(10, 9) + 7;
    let mut arr: Vec<u64> = Vec::new();
    arr.push(1);

    for i in 1..=money {
        let mut n = 0;
        for j in 0..num_coins {
            let prev = i - coins[j as usize];
            if prev < 0 {
                continue;
            }
            n += arr[prev as usize];
        }

        arr.push(n % MOD);
    }

    *arr.last().unwrap()
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
    let num_coins = first_line[0];
    let money = first_line[1];

    str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");

    let coin_values: Vec<i32> = str
        .split_whitespace()
        .map(|x| x.parse().unwrap_or(0))
        .collect();

    let mut m: HashMap<i32, i64> = HashMap::new();
    let result = coin_iter(money, num_coins, &coin_values);
    print!("{}", result);
}
