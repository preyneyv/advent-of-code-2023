use std::io;

pub fn main() {
    let stdin = io::stdin();
    let mut sum = 0;
    loop {
        let mut line = String::new();
        if stdin.read_line(&mut line).unwrap() == 0 {
            break;
        }
        let (id, observations) = line.split_once(':').unwrap();
        let has_illegal = observations.split(';').any(|obs| {
            obs.split(',').any(|segment| {
                let (count, label) = segment.trim().split_once(' ').unwrap();
                let count: u32 = count.parse().unwrap();
                match label {
                    "red" => count > 12,
                    "green" => count > 13,
                    "blue" => count > 14,
                    _ => unreachable!(),
                }
            })
        });
        if !has_illegal {
            sum += id[5..].parse::<u32>().unwrap();
        }
    }

    println!("{}", sum);
}
