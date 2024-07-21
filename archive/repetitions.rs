use std::io;

fn main() {
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");
    let s = str.as_bytes();

    let mut i = 0;
    let mut j = 0;
    let mut len = 0;
    let mut max = 0;
    loop {
        if s[i] == s[j] {
            len += 1;
        } else {
            i = j;
            if len > max {
                max = len;
            }
            len = 1;
        }

        j += 1;
        if j >= str.len() {
            break;
        }
    }

    print!("{}", max);
}
