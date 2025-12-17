use std::fs::File;
use std::io::Read;

fn part1((dial, zeros): (isize, isize), (direction, distance): (&str, isize)) -> (isize, isize) {
    let dist = match direction {
        "R" => distance,
        "L" => -distance,
        _ => unreachable!(),
    };
    let new_dial = (dial + dist).rem_euclid(100);

    (new_dial, zeros + (new_dial == 0) as isize)
}

fn part2((dial, zeros): (isize, isize), (direction, distance): (&str, isize)) -> (isize, isize) {
    let dist = match direction {
        "R" => distance,
        "L" => -distance,
        _ => unreachable!(),
    };
    let new_dial = dial + dist;

    let turns = match direction {
        "R" => new_dial / 100,
        "L" => {
            if new_dial < 0 {
                (new_dial / 100).abs() + if dial != 0 { 1 } else { 0 }
            } else {
                0
            }
        }
        _ => unreachable!(),
    };

    (
        new_dial.rem_euclid(100),
        zeros + (new_dial == 0) as isize + turns,
    )
}

fn main() {
    let mut input_file = File::open("input.txt").expect("Cannot open input file");
    let mut data = String::new();
    input_file
        .read_to_string(&mut data)
        .expect("Cannot read file");

    let lines = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (direction, distance_str) = line.split_at(1);
            let distance = str::parse::<isize>(distance_str).unwrap();
            (direction, distance)
        });

    let part1_zeros = lines.clone().fold((50, 0), part1).1;
    let part2_zeros = lines.fold((50, 0), part2).1;
    println!("{part1_zeros}");
    println!("{part2_zeros}");
}
