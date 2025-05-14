use regex::Regex;

pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r#"mul\([0-9]+,[0-9]+\)"#).unwrap();

    let mut sum = 0;
    for mut matched in re.find_iter(input).map(|x| x.as_str()) {
        matched = matched.trim_start_matches("mul(");
        matched = matched.trim_end_matches(")");
        let mut nums = matched.split(',');
        let first: i32 = nums.next().unwrap().parse().unwrap();
        let second: i32 = nums.next().unwrap().parse().unwrap();

        sum += first * second;
    }

    sum
}

pub fn part2(input: &str) -> i32 {
    let re = Regex::new(r#"(mul\([0-9]+,[0-9]+\))|(do\(\))|(don't\(\))"#).unwrap();

    let mut sum = 0;
    let mut matching = true;
    for matched in re
        .find_iter(input)
        .map(|x| Into::<Operation>::into(x.as_str()))
    {
        match matched {
            Operation::DO => matching = true,
            Operation::DONT => matching = false,
            Operation::MUL(num) => {
                if matching {
                    sum += num
                }
            }
        }
    }

    sum
}

enum Operation {
    DO,
    DONT,
    MUL(i32),
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "do()" => Self::DO,
            "don't()" => Self::DONT,
            _ => {
                let trimmed = value.trim_start_matches("mul(").trim_end_matches(")");
                let mut nums = trimmed.split(',');
                let first: i32 = nums.next().unwrap().parse().unwrap();
                let second: i32 = nums.next().unwrap().parse().unwrap();
                Self::MUL(first * second)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day3::{part1, part2};

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1_test() {
        assert_eq!(part1(INPUT), 161);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(INPUT2), 48);
    }
}
