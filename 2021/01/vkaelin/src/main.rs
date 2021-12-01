use std::fs;

fn main() {
    //part1();
    part2();
}

fn part1() {
    let filename = "../io/vkaelin.input";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<_> = content.lines().collect();

    let mut old = 0;
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
    let filename = "../io/vkaelin.input";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut lines = content.lines();

    let (mut window1, mut window2, mut window3) = (0, 0, 0);
    let (mut count1, mut count2, mut count3) = (0, 0, 0);
    let mut tmp: i32;
    let mut counter = 0;

    tmp = lines.next().unwrap().parse().unwrap();
    window1 += tmp;
    count1 += 1;

    tmp = lines.next().unwrap().parse().unwrap();
    window1 += tmp;
    window2 += tmp;
    count1 += 1;
    count2 += 1;

    while let Some(line) = lines.next() {
        let line: i32 = line.parse().expect("Line is not a number");

        if count1 != 3 {
            window1 += line;
            count1 += 1;
        }
        if count2 != 3 {
            window2 += line;
            count2 += 1;
        }
        if count3 != 3 {
            window3 += line;
            count3 += 1;
        }

        if count1 == 3 && count2 == 3 {
          if window2 > window1 {
            counter += 1;
          }
          count1 = 1;
          window1 = line;
        }

        if count2 == 3 && count3 == 3 {
          if window3 > window2 {
            counter += 1;
          }
          count2 = 1;
          window2 = line;
        }

        if count1 == 3 && count3 == 3 {
          if window1 > window3 {
            counter += 1;
          }
          count3 = 1;
          window3 = line;
        }
    }

    println!("Counter: {}", counter);
}
