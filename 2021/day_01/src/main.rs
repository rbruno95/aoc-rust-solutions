use std::io;

fn main() {
    let numbers = io::stdin()
        .lines()
        .map(|x| x.unwrap().trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    part_1(&numbers);
    part_2(&numbers);
}

fn part_1(numbers: &Vec<u32>) {
    println!("{}", increasing_pairs(numbers));
}

fn part_2(numbers: &Vec<u32>) {
    println!(
        "{}",
        increasing_pairs(
            &numbers
                .windows(3)
                .map(|triple| triple.iter().sum::<u32>())
                .collect::<Vec<_>>()
        )
    );
}

fn increasing_pairs(numbers: &Vec<u32>) -> usize {
    numbers.windows(2).filter(|pair| pair[0] < pair[1]).count()
}
