use std::collections::HashSet;

pub fn part1(input: &str) -> i32 {
    let mut board: Vec<Vec<Space>> = Vec::new();
    let mut guard: (usize, usize) = (0, 0);

    for (row, line) in input.lines().enumerate() {
        board.push(Vec::new());
        for (column, space) in line.chars().map(|x| Into::<Space>::into(x)).enumerate() {
            if space == Space::GUARD {
                guard = (row, column);
            }
            board.last_mut().unwrap().push(space);
        }
    }

    // for row in &board {
    //     for column in row {
    //         print!(
    //             "{}",
    //             match column {
    //                 Space::UNTRAVELED => '.',
    //                 Space::OBSTACLE => '#',
    //                 Space::GUARD => '^',
    //                 Space::TRAVELED => 'X',
    //             }
    //         )
    //     }
    //     println!("");
    // }

    let mut facing = Facing::UP;
    let mut traveled: HashSet<(usize, usize)> = HashSet::new();
    loop {
        let (vector_row, vector_column) = facing.movement_vector();
        let (guard_row, guard_column) = guard;
        let (moved_row, moved_column) = (
            (guard_row as isize + vector_row) as isize,
            (guard_column as isize + vector_column) as isize,
        );

        if moved_row < 0
            || moved_column < 0
            || moved_row >= board.len() as isize
            || moved_column >= board[0].len() as isize
        {
            traveled.insert((guard_row, guard_column));
            break;
        }

        if board[moved_row as usize][moved_column as usize] == Space::OBSTACLE {
            facing.rotate_right_90();
            continue;
        }

        board[guard_row][guard_column] = Space::TRAVELED;
        board[moved_row as usize][moved_column as usize] = Space::GUARD;
        traveled.insert((guard_row, guard_column));
        guard = (moved_row as usize, moved_column as usize);
    }

    return traveled.len() as i32;
}

enum Facing {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Facing {
    fn rotate_right_90(&mut self) {
        match self {
            Self::UP => *self = Self::RIGHT,
            Self::DOWN => *self = Self::LEFT,
            Self::LEFT => *self = Self::UP,
            Self::RIGHT => *self = Self::DOWN,
        }
    }

    fn movement_vector(&self) -> (isize, isize) {
        match self {
            Self::UP => (-1, 0),
            Self::DOWN => (1, 0),
            Self::LEFT => (0, -1),
            Self::RIGHT => (0, 1),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Space {
    UNTRAVELED,
    TRAVELED,
    OBSTACLE,
    GUARD,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '.' => Space::UNTRAVELED,
            '#' => Space::OBSTACLE,
            '^' => Space::GUARD,
            'X' => Space::TRAVELED,
            _ => unreachable!(),
        }
    }
}

pub fn part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1() {
        assert_eq!(crate::day6::part1(INPUT), 41);
    }
}
