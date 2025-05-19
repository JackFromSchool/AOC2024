use rust_aoc2024::day1;
use rust_aoc2024::day2;
use rust_aoc2024::day3;
use rust_aoc2024::day4;
use rust_aoc2024::day5;
use rust_aoc2024::day6;

fn main() {
    println!("Rust AOC2024!");
    println!("Day1:");
    println!("Part1 - {}", day1::part1(include_str!("./inputs/day1.txt")));
    println!("Part2 - {}", day1::part2(include_str!("./inputs/day1.txt")));
    println!("Day2:");
    println!("Part1 - {}", day2::part1(include_str!("./inputs/day2.txt")));
    println!("Part2 - {}", day2::part2(include_str!("./inputs/day2.txt")));
    println!("Day3:");
    println!("Part1 - {}", day3::part1(include_str!("./inputs/day3.txt")));
    println!("Part2 - {}", day3::part2(include_str!("./inputs/day3.txt")));
    println!("Day4:");
    println!("Part1 - {}", day4::part1(include_str!("./inputs/day4.txt")));
    println!("Part2 - {}", day4::part2(include_str!("./inputs/day4.txt")));
    println!("Day5:");
    println!("Part1 - {}", day5::part1(include_str!("./inputs/day5.txt")));
    println!("Part2 - {}", day5::part2(include_str!("./inputs/day5.txt")));
    println!("Day6:");
    println!("Part1 - {}", day6::part1(include_str!("./inputs/day6.txt")));
}
