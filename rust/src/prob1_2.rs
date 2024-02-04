use std::io;

pub fn main() {
    let stdin = io::stdin();
    let mut sum = 0u32;
    loop {
        let mut line = String::new();
        if stdin.read_line(&mut line).unwrap() == 0 {
            break;
        }
        let slice = line.as_bytes();

        let Some(x) = slice.iter().enumerate().find_map(|(from, ch)| {
            if &48 <= ch && ch <= &57 {
                return Some(ch - 48);
            }
            if slice[from..].starts_with(b"one") {
                return Some(1);
            }
            if slice[from..].starts_with(b"two") {
                return Some(2);
            }
            if slice[from..].starts_with(b"three") {
                return Some(3);
            }
            if slice[from..].starts_with(b"four") {
                return Some(4);
            }
            if slice[from..].starts_with(b"five") {
                return Some(5);
            }
            if slice[from..].starts_with(b"six") {
                return Some(6);
            }
            if slice[from..].starts_with(b"seven") {
                return Some(7);
            }
            if slice[from..].starts_with(b"eight") {
                return Some(8);
            }
            if slice[from..].starts_with(b"nine") {
                return Some(9);
            }
            None
        }) else {
            return;
        };

        let Some(y) = slice.iter().enumerate().rev().find_map(|(to, ch)| {
            if &48 <= ch && ch <= &57 {
                return Some(ch - 48);
            }
            if slice[..to].ends_with(b"one") {
                return Some(1);
            }
            if slice[..to].ends_with(b"two") {
                return Some(2);
            }
            if slice[..to].ends_with(b"three") {
                return Some(3);
            }
            if slice[..to].ends_with(b"four") {
                return Some(4);
            }
            if slice[..to].ends_with(b"five") {
                return Some(5);
            }
            if slice[..to].ends_with(b"six") {
                return Some(6);
            }
            if slice[..to].ends_with(b"seven") {
                return Some(7);
            }
            if slice[..to].ends_with(b"eight") {
                return Some(8);
            }
            if slice[..to].ends_with(b"nine") {
                return Some(9);
            }
            None
        }) else {
            return;
        };

        sum += u32::from(x) * 10 + u32::from(y);
    }
    println!("{}", sum);
}
