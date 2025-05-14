pub fn part1(input: &str) -> i32 {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut split = line.split_whitespace().take(2);
        list1.push(split.next().unwrap().parse().unwrap());
        list2.push(split.next().unwrap().parse().unwrap());
    }

    list1.sort();
    list2.sort();

    let mut sum = 0;

    for i in 0..list1.len() {
        let distance = i32::abs(list1[i] - list2[i]);
        sum += distance;
    }

    return sum;
}

pub fn part2(input: &str) -> i32 {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut split = line.split_whitespace().take(2);
        list1.push(split.next().unwrap().parse().unwrap());
        list2.push(split.next().unwrap().parse().unwrap());
    }

    list1.sort();
    list2.sort();

    list1.iter().fold(0, |mut acc, elem| {
        acc += *elem
            * list2
                .iter()
                .filter(|x| *x == elem)
                .collect::<Vec<_>>()
                .len() as i32;
        acc
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(crate::day1::part1(input), 11);
    }

    #[test]
    fn part2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(crate::day1::part2(input), 31);
    }
}
