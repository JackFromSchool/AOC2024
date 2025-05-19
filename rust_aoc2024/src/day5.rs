use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> i32 {
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut taking_rules = true;
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            taking_rules = false;
            continue;
        }

        if taking_rules {
            let mut split = line.split('|');
            let first = split.next().unwrap().parse::<i32>().unwrap();
            let after = split.next().unwrap().parse::<i32>().unwrap();

            rules.entry(first).and_modify(|x| x.push(after)).or_insert({
                let mut new_vec = Vec::new();
                new_vec.push(after);
                new_vec
            });
        } else {
            let mut updates_set = HashSet::new();
            let mut updates = Vec::new();
            let mut valid = true;

            'outer: for update in line.split(',').map(|x| x.parse::<i32>().unwrap()) {
                if let Some(after) = rules.get(&update) {
                    for item in after {
                        if updates_set.contains(item) {
                            valid = false;
                            break 'outer;
                        }
                    }
                }

                updates_set.insert(update);
                updates.push(update);
            }

            if valid {
                sum += updates[updates.len() / 2];
            }
        }
    }

    sum
}

pub fn part2(input: &str) -> i32 {
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();

    let mut taking_rules = true;
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            taking_rules = false;
            continue;
        }

        if taking_rules {
            let mut split = line.split('|');
            let first = split.next().unwrap().parse::<i32>().unwrap();
            let after = split.next().unwrap().parse::<i32>().unwrap();

            rules
                .entry(first)
                .and_modify(|x| {
                    x.insert(after);
                })
                .or_insert({
                    let mut new_set = HashSet::new();
                    new_set.insert(after);
                    new_set
                });
        } else {
            let mut updates_set = HashSet::new();
            let mut valid = true;

            'outer: for update in line.split(',').map(|x| x.parse::<i32>().unwrap()) {
                if let Some(after) = rules.get(&update) {
                    if !after.is_disjoint(&updates_set) {
                        valid = false;
                        break 'outer;
                    }
                }

                updates_set.insert(update);
            }

            if valid {
                continue;
            }

            let mut updates: HashSet<i32> =
                line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();

            let mut sorted = Vec::new();
            while !updates.is_empty() {
                for item in updates.iter().cloned().collect::<Vec<i32>>() {
                    if let Some(rule) = rules.get(&item) {
                        if rule.is_disjoint(&updates) {
                            updates.remove(&item);
                            sorted.push(item);
                            break;
                        }
                    } else {
                        updates.remove(&item);
                        sorted.push(item);
                        break;
                    }
                }
            }

            sorted.reverse();
            sum += sorted[sorted.len() / 2];
        }
    }

    sum
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1() {
        assert_eq!(crate::day5::part1(INPUT), 143);
    }

    #[test]
    fn part2() {
        assert_eq!(crate::day5::part2(INPUT), 123);
    }
}
