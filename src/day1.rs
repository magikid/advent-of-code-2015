use std::fs;

pub fn p1() -> i32 {
    let puzzle_input = get_puzzle_input();
    count_floors(puzzle_input)
}

pub fn p2() -> usize {
    let puzzle_input = get_puzzle_input();
    first_basement(puzzle_input)
}

fn get_puzzle_input() -> String {
    fs::read_to_string("inputs/d1p1.txt")
        .expect("Somthing went wrong reading the file inputs/d1p1.txt")
}

fn count_floors(elevator_selections: String) -> i32 {
    let up_floors = elevator_selections.matches('(').count() as i32;
    let down_floors = elevator_selections.matches(')').count() as i32;

    up_floors - down_floors
}

fn first_basement(elevator_selections: String) -> usize {
    let mut current_floor = 0;

    for (i, char) in elevator_selections.chars().enumerate() {
        trace!("current_floor: {}, i: {}", current_floor, i);
        if char == '(' {
            current_floor += 1;
        } else {
            current_floor -= 1;
        }

        if current_floor == -1 {
            return i + 1;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn it_counts_zero_floors() {
        let test_input = String::from("(())");
        let floors_traversed = count_floors(test_input);
        assert_eq!(0, floors_traversed)
    }

    #[test]
    fn it_counts_different_zero_floors() {
        let test_input = String::from("()()");
        let floors_traversed = count_floors(test_input);
        assert_eq!(0, floors_traversed)
    }

    #[test]
    fn it_counts_three_floors_simple() {
        let test_input = String::from("(((");
        let floors_traversed = count_floors(test_input);
        assert_eq!(3, floors_traversed)
    }

    #[test]
    fn it_counts_three_floors_complex() {
        let test_input = String::from("(()(()(");
        let floors_traversed = count_floors(test_input);
        assert_eq!(3, floors_traversed)
    }

    #[test]
    fn it_counts_three_floors_more_complex() {
        let test_input = String::from("))(((((");
        let floors_traversed = count_floors(test_input);
        assert_eq!(3, floors_traversed)
    }

    #[test]
    fn it_counts_basement_floors_simple() {
        let test_input = String::from("())");
        let floors_traversed = count_floors(test_input);
        assert_eq!(-1, floors_traversed)
    }

    #[test]
    fn it_counts_basement_floors_complex() {
        let test_input = String::from("))(");
        let floors_traversed = count_floors(test_input);
        assert_eq!(-1, floors_traversed)
    }

    #[test]
    fn it_counts_three_basement_floors_simple() {
        let test_input = String::from(")))");
        let floors_traversed = count_floors(test_input);
        assert_eq!(-3, floors_traversed)
    }

    #[test]
    fn it_counts_three_basement_floors_complex() {
        let test_input = String::from(")())())");
        let floors_traversed = count_floors(test_input);
        assert_eq!(-3, floors_traversed)
    }

    #[test]
    fn it_finds_basement_simple() {
        init();

        let test_input = String::from(")");
        let basement_position = first_basement(test_input);
        assert_eq!(1, basement_position)
    }

    #[test]
    fn it_finds_basement_complex() {
        init();

        let test_input = String::from("()())");
        let basement_position = first_basement(test_input);
        assert_eq!(5, basement_position)
    }
}
