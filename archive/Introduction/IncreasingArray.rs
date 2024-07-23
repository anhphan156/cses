use std::io;

fn main() {
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");

    let n: usize = str.trim().parse().expect("");

    if n == 1 {
        print!("{}", 0);
        return;
    }

    str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");

    let mut s: Vec<String> = str.split_whitespace().map(|k| k.to_string()).collect();

    let mut i = 1;
    let mut step: u64 = 0;
    let mut a: u64;
    let mut b: u64;
    loop {
        a = s[i - 1].parse().expect("");
        b = s[i].parse().expect("");
        if a > b {
            step += a - b;
            s[i] = a.to_string();
        }

        i += 1;
        if i >= n {
            break;
        }
    }
    print!("{}", step);
}
