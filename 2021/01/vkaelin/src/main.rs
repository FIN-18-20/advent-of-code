use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let filename = "../io/vkaelin.input";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<_> = content.lines().collect();

    let mut old= 0;
    let mut counter = 0;

    for (i, line) in lines.iter().enumerate() {
        let line: i32 = line.parse().expect("Error: line is not a number");

        if i > 0 && line > old {
          counter += 1;
        }
        old = line;
    }

    println!("Counter: {}", counter);
}

fn part2() {

}
