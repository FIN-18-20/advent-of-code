use std::fs;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input_file = "../io/lothindir.input";

    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let lines: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    for i in 0..lines[0].len() {
        let mut zero_count = 0;
        let mut one_count = 0;

        for j in 0..lines.len() {
            if lines[j][i] == '0' {
                zero_count += 1;
            } else {
                one_count += 1;
            }
        }
        if zero_count > one_count {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        } else {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        }
    }
    let gamma_rate = u32::from_str_radix(gamma_rate.as_str(), 2).unwrap_or(0);
    let epsilon_rate = u32::from_str_radix(epsilon_rate.as_str(), 2).unwrap_or(0);

    println!(
        "[1] Gamma rate: {}, Epsilon rate: {} => {}",
        &gamma_rate,
        &epsilon_rate,
        &gamma_rate * &epsilon_rate
    );
}

fn part_two() {
    let input_file = "../io/example.input";

    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let lines: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();

    let mut oxygen_rate = 0;
    let mut co2_rate = 0;

    let mut zero_count = 0;
    let mut one_count = 0;

    for j in 0..lines.len() {
        print!("{} ", &lines[j][0]);
        if lines[j][0] == '0' {
            zero_count += 1;
        } else {
            one_count += 1;
        }
    }
    println!("0:{} - 1:{}", &zero_count, &one_count);

    if zero_count > one_count {
        oxygen_rate = filter_by_bit_criteria(&lines, '0', 0, true);
        co2_rate = filter_by_bit_criteria(&lines, '1', 0, false);
    } else {
        oxygen_rate = filter_by_bit_criteria(&lines, '1', 0, false);
        co2_rate = filter_by_bit_criteria(&lines, '0', 0, true);
    }

    println!(
        "[2] Oxygen rate: {}, CO2 rate: {} => {}",
        &oxygen_rate,
        &co2_rate,
        &oxygen_rate * &co2_rate
    );
}

fn filter_by_bit_criteria(array: &Vec<Vec<char>>, bit: char, position: usize, asc: bool) -> u32 {
    let mut array_copy = array.clone();
    if array_copy.len() > 1 {
        array_copy = array_copy
            .into_iter()
            .filter(|line| line[position] == bit)
            .collect();

        if position + 1 < array[0].len() {
            let mut zero_count = 0;
            let mut one_count = 0;

            for j in 0..array_copy.len() {
                // print!("{} ", &array_copy[j][position]);
                if array_copy[j][position + 1] == '0' {
                    zero_count += 1;
                } else {
                    one_count += 1;
                }
            }
            let mut bit = '0';
            let mut asc = true;
            if zero_count < one_count {
                bit = '1';
                asc = !asc;
            }
            println!(
                " [0:{} - 1:{}] - Position:{} - {:?}",
                &zero_count,
                &one_count,
                &position + 1,
                &array_copy
            );
            return filter_by_bit_criteria(&array_copy, bit, position + 1, asc);
        }
    }

    let mut value = String::new();
    array_copy[0].iter().for_each(|c| value.push(*c));
    u32::from_str_radix(value.as_str(), 2).unwrap_or(0)
}
