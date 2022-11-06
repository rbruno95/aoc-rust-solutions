fn main() {
    let binary_numbers = std::io::stdin()
        .lines()
        .map(|x| x.expect("Issue obtaining the line from stdin"))
        .collect::<Vec<String>>();

    part_01(&binary_numbers);
    part_02(&binary_numbers);
}

fn part_01(binary_numbers: &Vec<String>) {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    let size = 12;
    let len = binary_numbers.len();

    for i in 0..size {
        let one_counter = binary_numbers
            .iter()
            .filter(|x| x.as_bytes()[i] == b'1')
            .count();

        match one_counter * 2 >= len {
            true => {
                gamma_rate = gamma_rate * 2 + 1;
                epsilon_rate *= 2;
            }
            false => {
                gamma_rate *= 2;
                epsilon_rate = epsilon_rate * 2 + 1;
            }
        }
    }

    println!("{}", gamma_rate * epsilon_rate);
}

fn part_02(binary_numbers: &Vec<String>) {
    let size = 12;

    let mut pattern = String::new();
    let mut previous_pattern_len = binary_numbers.len();

    for _ in 0..size {
        pattern.push_str("1");

        let pattern_counting = binary_numbers
            .iter()
            .filter(|x| x.starts_with(&pattern))
            .count();

        match pattern_counting * 2 >= previous_pattern_len {
            true => {
                previous_pattern_len = pattern_counting;
            }
            false => {
                previous_pattern_len -= pattern_counting;
                pattern.pop();
                pattern.push_str("0");
            }
        }

        if previous_pattern_len == 1 {
            pattern = binary_numbers
                .iter()
                .find(|x| x.starts_with(&pattern))
                .unwrap()
                .to_owned();
            break;
        }
    }

    let oxygen_generator_rating =
        i32::from_str_radix(&pattern, 2).expect("The string is not a binary number");

    pattern = String::new();
    previous_pattern_len = binary_numbers.len();

    for _ in 0..size {
        pattern.push_str("1");

        let pattern_counting = binary_numbers
            .iter()
            .filter(|x| x.starts_with(&pattern))
            .count();

        match pattern_counting * 2 < previous_pattern_len {
            true => {
                previous_pattern_len = pattern_counting;
            }
            false => {
                previous_pattern_len -= pattern_counting;
                pattern.pop();
                pattern.push_str("0");
            }
        }

        if previous_pattern_len == 1 {
            pattern = binary_numbers
                .iter()
                .find(|x| x.starts_with(&pattern))
                .unwrap()
                .to_owned();
            break;
        }
    }

    let c02_scrubber_rating =
        i32::from_str_radix(&pattern, 2).expect("The string is not a binary number");

    println!("{}", oxygen_generator_rating * c02_scrubber_rating);
}
