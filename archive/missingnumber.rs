use std::io;

fn main() {
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");
    let n: u32 = str.trim().parse().expect("");

    str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");

    let mut sum1 = n;
    let mut sum2: u32 = 0;
    let mut nn: u32;
    for (i, s) in str.split_whitespace().enumerate() {
        sum1 += i as u32 + 1;
        nn = s.trim().parse().expect("");
        sum2 += nn;
    }

    print!("{}", sum1 - sum2);
}
