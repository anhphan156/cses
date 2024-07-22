#![allow(dead_code)]
use std::{collections::HashMap, io};

fn dice_rec(n: i32, m: &mut HashMap<i32, u64>) -> u64 {
    if m.contains_key(&n) {
        return *m.get(&n).unwrap();
    }
    if n < 0 {
        return 0;
    }
    if n == 0 {
        return 1;
    }

    const MOD: u64 = u64::pow(10, 9) + 7;
    let result: u64 = (dice_rec(n - 1, m)
        + dice_rec(n - 2, m)
        + dice_rec(n - 3, m)
        + dice_rec(n - 4, m)
        + dice_rec(n - 5, m)
        + dice_rec(n - 6, m))
        % MOD;

    m.insert(n, result);

    result
}

fn dice_iter(n: i32) -> u64 {
    const MOD: u64 = u64::pow(10, 9) + 7;

    let mut m: Vec<u64> = Vec::new();
    m.push(1);

    for i in 1..=n {
        let mut sum = 0;
        for j in 1..=6 {
            let prev = i - j;
            if prev >= 0 {
                sum += m[prev as usize];
            }
        }
        m.push(sum % MOD);
    }
    *m.last().unwrap()
}

fn main() {
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");

    let mut m: HashMap<i32, u64> = HashMap::new();
    let n = str.trim().parse().expect("");
    let result = dice_rec(n, &mut m);
    //let result = dice_iter(n);
    print!("{}", result);
}
