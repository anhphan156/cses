use std::io;

fn main() {
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");
    let mut n: u64 = str.trim().parse().expect("Failed to parse");

    loop {
        print!("{} ", n);

        if n <= 1 {
            break;
        }

        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = n * 3 + 1;
        }
    }
}
