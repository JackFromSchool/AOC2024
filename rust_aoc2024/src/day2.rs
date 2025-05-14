pub fn part1(input: &str) -> i32 {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        reports.push(line.split_whitespace().fold(Vec::new(), |mut acc, elem| {
            acc.push(elem.parse().unwrap());
            return acc;
        }));
    }

    let mut safe_reports = 0;
    for report in reports {
        let mut previous_level = report[0];
        let mut safe = true;
        let decreasing = report[1] - previous_level < 0;
        for i in 1..report.len() {
            let difference = report[i] - previous_level;
            if i32::abs(difference) == 0
                || i32::abs(difference) > 3
                || difference < 0 && !decreasing
                || difference > 0 && decreasing
            {
                safe = false;
                break;
            }
            previous_level = report[i];
        }

        if safe {
            safe_reports += 1;
        }
    }

    safe_reports
}

pub fn part2(input: &str) -> i32 {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        reports.push(line.split_whitespace().fold(Vec::new(), |mut acc, elem| {
            acc.push(elem.parse().unwrap());
            return acc;
        }));
    }

    let mut safe_reports = 0;
    for report in reports {
        let mut safe = false;
        for skip in 0..report.len() {
            let mut previous_level = None;
            let mut decreasing_level = None;
            let mut skip_safe = true;
            for i in (0..report.len()).filter(|x| *x != skip) {
                if let Some(previous) = previous_level {
                    let difference = report[i] - previous;

                    let decreasing = match decreasing_level {
                        Some(x) => x,
                        None => {
                            decreasing_level = Some(report[i] - previous < 0);
                            report[i] - previous < 0
                        }
                    };

                    if i32::abs(difference) == 0
                        || i32::abs(difference) > 3
                        || difference < 0 && !decreasing
                        || difference > 0 && decreasing
                    {
                        skip_safe = false;
                        break;
                    }

                    previous_level = Some(report[i]);
                } else {
                    previous_level = Some(report[i]);
                }
            }

            if skip_safe {
                safe = true;
                break;
            }
        }

        if safe {
            safe_reports += 1;
        }
    }

    safe_reports
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1() {
        assert_eq!(crate::day2::part1(INPUT), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(crate::day2::part2(INPUT), 4);
    }
}
