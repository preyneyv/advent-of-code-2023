use std::io;

pub fn main() {
    let stdin = io::stdin();
    let mut sum = 0u32;
    loop {
        let mut line = String::new();
        if stdin.read_line(&mut line).unwrap() == 0 {
            break;
        }
        let x = line.chars().find(|t| t.is_numeric()).unwrap();
        let y = line.chars().rfind(|t| t.is_numeric()).unwrap();
        sum += (x.to_digit(10).unwrap()) * 10 + y.to_digit(10).unwrap();
    }
    println!("{}", sum);
}
