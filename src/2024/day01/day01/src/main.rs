use core::time;
use std::time::Instant;

fn main() {
    let original_pairs: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut firsts: Vec<i32> = original_pairs.iter().map(|x| *x.first().unwrap()).collect();
    let mut seconds: Vec<i32> = original_pairs.iter().map(|x| *x.get(1).unwrap()).collect();

    firsts.sort();
    seconds.sort();

    let it = firsts.iter().zip(seconds.iter());

    //println!("{:?}", original_pairs);
    //println!("{:?}", firsts);

    let time_part1 = Instant::now();
    let part1: i32 = it.enumerate().map(|(_, (a, b))| (a - b).abs()).sum();
    println!("{:?} in {:?}", part1, time_part1.elapsed());

    let time_part2 = Instant::now();
    let part2: i32 = firsts
        .iter()
        .map(|x| {
            let count = seconds.iter().filter(|y| *y == x).count() as i32;
            x * count
        })
        .sum();

    println!("{:?} in {:?}", part2, time_part2.elapsed());
}
