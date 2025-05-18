pub fn part1(input: &str) -> i32 {
    let mut puzzle: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        puzzle.push(Vec::new());
        for character in line.chars() {
            puzzle.last_mut().unwrap().push(character);
        }
    }

    let mut count = 0;
    for row in 0..puzzle.len() {
        for column in 0..puzzle[0].len() {
            count += count_xmas(&puzzle, (row as isize, column as isize));
        }
    }

    count
}

fn count_xmas(puzzle: &Vec<Vec<char>>, coord: (isize, isize)) -> i32 {
    let hunt = vec!['M', 'A', 'S'];
    let mut coords = vec![
        (1, 0),
        (1, 1),
        (-1, 0),
        (-1, -1),
        (0, 1),
        (0, -1),
        (1, -1),
        (-1, 1),
    ];
    let (row, column) = coord;

    if puzzle[row as usize][column as usize] != 'X' {
        return 0;
    }

    let mut count = 0;
    while !coords.is_empty() {
        let (off_row, off_column) = coords.pop().unwrap();

        let mut hunting_row = row;
        let mut hunting_column = column;
        let mut passing = true;
        for hunting in 0..hunt.len() {
            hunting_row += off_row;
            hunting_column += off_column;

            if hunting_row as usize >= puzzle.len()
                || hunting_row < 0
                || hunting_column as usize >= puzzle[0].len()
                || hunting_column < 0
            {
                passing = false;
                break;
            }

            if puzzle[hunting_row as usize][hunting_column as usize] != hunt[hunting] {
                passing = false;
                break;
            }
        }

        if passing {
            count += 1;
        }
    }

    count
}

pub fn part2(input: &str) -> i32 {
    let possiblilities: Vec<Vec<char>> = vec![
        vec!['M', 'M', 'S', 'S'],
        vec!['S', 'M', 'M', 'S'],
        vec!['S', 'S', 'M', 'M'],
        vec!['M', 'S', 'S', 'M'],
    ];
    let mut puzzle: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        puzzle.push(Vec::new());
        for character in line.chars() {
            puzzle.last_mut().unwrap().push(character);
        }
    }

    let mut count = 0;
    for row in 0..puzzle.len() {
        for column in 0..puzzle[0].len() {
            if puzzle[row][column] != 'A' {
                continue;
            }

            if let Some(cross_segment) = get_cross_segment(&puzzle, (row as isize, column as isize))
            {
                for possiblity in &possiblilities {
                    if cross_segment == *possiblity {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }

    count
}

fn get_cross_segment(puzzle: &Vec<Vec<char>>, coord: (isize, isize)) -> Option<Vec<char>> {
    let (row, column) = coord;
    let (top_row, left_column) = (row - 1, column - 1);
    let (bottom_row, right_column) = (row + 1, column + 1);

    if top_row < 0
        || left_column < 0
        || bottom_row as usize >= puzzle.len()
        || right_column as usize >= puzzle[0].len()
    {
        return None;
    }

    let mut cross_segment: Vec<char> = Vec::new();
    let mut coords = vec![(-1, -1), (-1, 1), (1, 1), (1, -1)];

    while !coords.is_empty() {
        let (off_row, off_column) = coords.pop().unwrap();

        cross_segment.push(puzzle[(row + off_row) as usize][(column + off_column) as usize]);
    }

    Some(cross_segment)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1() {
        assert_eq!(crate::day4::part1(INPUT), 18)
    }

    #[test]
    fn part2() {
        assert_eq!(crate::day4::part2(INPUT), 9)
    }
}
