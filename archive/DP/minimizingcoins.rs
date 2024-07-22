#![allow(unused)]
use std::{cmp::min, collections::HashMap, io};

fn coin_rec(money: i32, num_coins: i32, coins: &Vec<i32>, m: &mut HashMap<i32, i64>) -> i64 {
    if m.contains_key(&money) {
        return *m.get(&money).unwrap();
    }

    if money < 0 {
        return -1;
    }
    if money == 0 {
        return 0;
    }

    let mut min_coin = i64::MAX;
    for i in 0..num_coins {
        let a = 1 + coin_rec(money - coins[i as usize], num_coins, coins, m);
        if a > 0 {
            min_coin = min(min_coin, a);
        }
    }

    if min_coin == i64::MAX {
        min_coin = -1;
    }
    m.insert(money, min_coin);

    min_coin
}

fn coin_iter(money: i32, num_coins: i32, coins: &Vec<i32>) -> i64 {
    let mut arr: Vec<i64> = Vec::new();
    arr.push(0);

    for i in 1..=money {
        let mut min_val = i64::MAX;
        for j in 0..num_coins {
            let prev = i - coins[j as usize];
            if prev < 0 {
                continue;
            }
            let n = 1 + arr[prev as usize];
            if n > 0 {
                min_val = min(min_val, n);
            }
        }
        if min_val == i64::MAX {
            min_val = -1;
        }

        arr.push(min_val);
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
    //let result = coin_rec(money, num_coins, &coin_values, &mut m);
    let result = coin_iter(money, num_coins, &coin_values);
    print!("{}", result);
}
