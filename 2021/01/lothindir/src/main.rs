use std::fs;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input_file = "../io/lothindir.input";

    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let mut lines = contents.lines();

    let mut old_measurement: i32 = lines
        .next()
        .expect("File is empty")
        .parse()
        .expect("Input is not a number");
    let mut measurement_increases = 0;
    while let Some(l) = lines.next() {
        let l: i32 = l.parse().expect("Measurement is not a number");
        if l > old_measurement {
            measurement_increases += 1;
        }
        old_measurement = l;
    }

    println!("Part one increases : {}", measurement_increases);
}

fn part_two() {
    let input_file = "../io/lothindir.input";

    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let mut lines = contents.lines();

    let mut m = [0, 0, 0];
    let mut counter = 0;
    let mut old_sum = 0;
    let mut increase = 0;

    while let Some(s) = lines.next() {
        let line: i32 = s.parse().unwrap();
        m[2] = m[1];
        m[1] = m[0];
        m[0] = line;
        counter += 1;

        if counter > 3 {
            let sum: i32 = m.iter().sum();
            if sum > old_sum {
                increase += 1;
            }
            old_sum = sum;
        }
    }
    println!("Part two increase : {}", increase);
}
