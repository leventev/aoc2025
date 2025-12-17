use std::fs::File;
use std::io::Read;

fn part1((start, end): (usize, usize)) -> usize {
    let mut num = start;
    let mut sum: usize = 0;
    while num <= end {
        let digits = num.ilog10();
        if digits % 2 == 0 {
            num = usize::pow(10, digits + 1);
            continue;
        };
        let log_half = usize::pow(10, digits / 2 + 1);
        let left = num / log_half;
        let right = num % log_half;
        if left == right {
            sum += num;
        }

        num += 1;
    }

    sum
}

fn part2((start, end): (usize, usize)) -> usize {
    let mut num = start;
    let mut sum: usize = 0;
    while num <= end {
        let digits = num.ilog10() + 1;

        'outer: for n in 1..digits / 2 + 1 {
            if digits % n != 0 {
                continue;
            }

            let segments = digits / n;
            let divisor = usize::pow(10, n);
            let rightmost_segment = num % divisor;
            for k in 1..segments {
                let val = num / (divisor.pow(k)) % divisor;
                if rightmost_segment != val {
                    continue 'outer;
                }
            }

            sum += num;
            break;
        }

        num += 1;
    }

    sum
}

fn main() {
    let mut input_file = File::open("input.txt").expect("Cannot open input file");
    let mut data = String::new();
    input_file
        .read_to_string(&mut data)
        .expect("Cannot read file");

    let ranges = data.split(',').map(|line| {
        let mut bounds = line.trim_end().split('-');
        let start = str::parse::<usize>(bounds.next().unwrap()).unwrap();
        let end = str::parse::<usize>(bounds.next().unwrap()).unwrap();
        (start, end)
    });

    let part1_sum: usize = ranges.clone().map(part1).sum();
    let part2_sum: usize = ranges.map(part2).sum();
    println!("{part1_sum}");
    println!("{part2_sum}");
}
