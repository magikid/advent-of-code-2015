use std::fs;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn p1() -> usize {
    let puzzle_input = get_puzzle_input();
    count_steps(vec![Direction::Right]);
    0
}

pub fn p2() -> usize {
    let puzzle_input = get_puzzle_input();
    0
}

pub fn count_steps(directions: Vec<Direction>) -> usize {
    let mut current_x = 0;
    let mut current_y = 0;
    let mut delta_x = 0;
    let mut delta_y = 0;

    for direction in directions.iter() {
        trace!("current x: {}, current y: {}", current_x, current_y);

        match direction {
            Direction::Right => delta_x = 1,
            Direction::Left => delta_x = -1,
            Direction::Up => delta_y = 1,
            Direction::Down => delta_y = -1,
        }

        trace!("delta x: {}, delta y: {}", delta_x, delta_y);
        current_x = current_x + delta_x;
        current_y = current_y + delta_y;
        trace!("new x: {}, new y: {}", current_x, current_y)
    }
    0
}

fn get_puzzle_input() -> Vec<Direction> {
    let file = "d3p1.txt";
    let file_string = fs::read_to_string(format!("inputs/{}", file))
        .expect(format!("Somthing went wrong reading the file inputs/{}", file).as_str());
    file_string
        .chars()
        .map(|char| match char {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!("Unsupported direction found: {}", char),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_test_first_example() {
        assert_eq!(0, p1())
    }

    #[test]
    fn second_test() {
        assert_eq!(0, p2())
    }
}
