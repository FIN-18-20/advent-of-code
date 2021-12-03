use std::fs;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input_file = "../io/lothindir.input";

    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let mut lines = contents.lines();

    let mut depth = 0;
    let mut h_position = 0;

    while let Some(l) = lines.next() {
        let mut split = l.split(" ");
        match split.next() {
            Some("forward") => h_position += split.next().unwrap_or("0").parse::<i32>().unwrap(),
            Some("down") => depth += split.next().unwrap_or("0").parse::<i32>().unwrap(),
            Some("up") => depth -= split.next().unwrap_or("0").parse::<i32>().unwrap(),
            Some(_) => break,
            None => break,
        }
    }

    println!(
        "[1] Depth: {}, position: {} => {}",
        &depth,
        &h_position,
        &depth * &h_position
    );
}

fn part_two() {
    let input_file = "../io/lothindir.input";

    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let mut lines = contents.lines();

    let mut depth = 0;
    let mut h_position = 0;
    let mut aim = 0;

    while let Some(l) = lines.next() {
        let mut split = l.split(" ");
        match split.next() {
            Some("forward") => {
                let value = split.next().unwrap_or("0").parse::<i32>().unwrap();
                h_position += value;
                depth += value * aim;
            }
            Some("down") => aim += split.next().unwrap_or("0").parse::<i32>().unwrap(),
            Some("up") => aim -= split.next().unwrap_or("0").parse::<i32>().unwrap(),
            Some(_) => break,
            None => break,
        }
    }

    println!(
        "[2] Depth: {}, position: {} => {}",
        &depth,
        &h_position,
        &depth * &h_position
    );
}
