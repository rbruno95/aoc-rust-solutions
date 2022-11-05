use std::io;

fn main() {
    let commands: Vec<Vec<String>> = io::stdin()
        .lines()
        .map(|line| {
            line.expect("Issue obtaining the line from stdin")
                .trim()
                .split_whitespace()
                .map(|token| token.to_owned())
                .collect()
        })
        .collect();

    part_1(&commands);
    part_2(&commands);
}

fn part_1(commands: &Vec<Vec<String>>) {
    let mut depth = 0;
    let mut horizontal_position = 0;

    commands.iter().for_each(|command| {
        let x = command[1].parse::<i32>().expect("not a valid integer");

        match &command[0][..] {
            "forward" => horizontal_position += x,
            "down" => depth += x,
            "up" => depth -= x,
            _ => panic!("not expected command"),
        }
    });

    println!("{}", depth * horizontal_position);
}

fn part_2(commands: &Vec<Vec<String>>) {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    commands.iter().for_each(|command| {
        let x = command[1].parse::<i32>().expect("not a valid integer");

        match &command[0][..] {
            "forward" => {
                horizontal_position += x;
                depth += aim * x;
            }
            "down" => aim += x,
            "up" => aim -= x,
            _ => panic!("not expected command"),
        }
    });

    println!("{}", depth * horizontal_position);
}
