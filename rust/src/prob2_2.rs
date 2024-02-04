use std::io;

struct Observation {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn main() {
    let stdin = io::stdin();
    let mut sum = 0;
    loop {
        let mut line = String::new();
        if stdin.read_line(&mut line).unwrap() == 0 {
            break;
        }
        let (_, observations) = line.split_once(':').unwrap();
        let out = observations.split(';').fold(
            Observation {
                blue: 0,
                red: 0,
                green: 0,
            },
            |mut acc, obs| {
                for segment in obs.split(',') {
                    let (count, label) = segment.trim().split_once(' ').unwrap();
                    let count: u32 = count.parse().unwrap();
                    match label {
                        "red" => acc.red = std::cmp::max(acc.red, count),
                        "green" => acc.green = std::cmp::max(acc.green, count),
                        "blue" => acc.blue = std::cmp::max(acc.blue, count),
                        _ => unreachable!(),
                    };
                }
                acc
            },
        );
        sum += out.red * out.green * out.blue;
    }

    println!("{}", sum);
}
