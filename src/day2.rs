use std::fs;

struct PresentBox {
    length: usize,
    width: usize,
    height: usize,
}

pub fn p1() -> usize {
    let puzzle_input = get_puzzle_input();
    puzzle_input
        .into_iter()
        .map(calculate_needed_wrapping_paper)
        .sum()
}

pub fn p2() -> usize {
    let puzzle_input = get_puzzle_input();
    puzzle_input.into_iter().map(calculate_needed_ribbon).sum()
}

fn get_puzzle_input() -> Vec<PresentBox> {
    let file = "d2p1.txt";
    let contents = fs::read_to_string(format!("inputs/{}", file))
        .expect(format!("Somthing went wrong reading the file inputs/{}", file).as_str());
    contents
        .split('\n')
        .take_while(|&line| line != "")
        .map(|line| {
            let parts: Vec<&str> = line.split('x').collect();
            trace!("parts: {:?}", parts);

            PresentBox {
                length: parts.get(0).unwrap().parse::<usize>().unwrap(),
                width: parts.get(1).unwrap().parse::<usize>().unwrap(),
                height: parts.get(2).unwrap().parse::<usize>().unwrap(),
            }
        })
        .collect()
}

fn calculate_needed_wrapping_paper(present_box: PresentBox) -> usize {
    let smallest_side = smallest_side(present_box.length, present_box.width, present_box.height);
    let box_area = box_area(present_box.length, present_box.width, present_box.height);

    box_area + smallest_side
}

fn smallest_side(length: usize, width: usize, height: usize) -> usize {
    let first_side = length * width;
    let second_side = length * height;
    let third_side = width * height;
    let mut side_areas = vec![first_side, second_side, third_side];

    side_areas.sort();
    *side_areas.first().unwrap()
}

fn box_area(length: usize, width: usize, height: usize) -> usize {
    2 * length * width + 2 * width * height + 2 * height * length
}

fn calculate_needed_ribbon(present_box: PresentBox) -> usize {
    let ribbon_wrap = ribbon_wrap_length(present_box.length, present_box.width, present_box.height);
    let bow = bow_length(present_box.length, present_box.width, present_box.height);

    ribbon_wrap + bow
}

fn ribbon_wrap_length(length: usize, width: usize, height: usize) -> usize {
    let first_face = 2 * length + 2 * width;
    let second_face = 2 * height + 2 * width;
    let third_face = 2 * height + 2 * length;
    let mut faces = vec![first_face, second_face, third_face];

    faces.sort();
    *faces.first().unwrap()
}

fn bow_length(length: usize, width: usize, height: usize) -> usize {
    length * width * height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_test_first_example() {
        let present_box = PresentBox {
            length: 2,
            width: 3,
            height: 4,
        };

        let wrapping_paper_needed = calculate_needed_wrapping_paper(present_box);

        assert_eq!(58, wrapping_paper_needed)
    }

    #[test]
    fn full_test_second_example() {
        let present_box = PresentBox {
            length: 1,
            width: 1,
            height: 10,
        };

        let wrapping_paper_needed = calculate_needed_wrapping_paper(present_box);

        assert_eq!(43, wrapping_paper_needed)
    }

    #[test]
    fn it_calcs_smallest_side() {
        let smallest_side = smallest_side(2, 3, 4);
        assert_eq!(6, smallest_side)
    }

    #[test]
    fn it_calcs_different_smallest_side() {
        let smallest_side = smallest_side(1, 1, 10);
        assert_eq!(1, smallest_side)
    }

    #[test]
    fn box_area_example_1() {
        let area = box_area(2, 3, 4);
        assert_eq!(52, area)
    }

    #[test]
    fn box_area_example_2() {
        let area = box_area(1, 1, 10);
        assert_eq!(42, area)
    }

    #[test]
    fn bow_length_example_1() {
        let length = bow_length(2, 3, 4);
        assert_eq!(24, length)
    }

    #[test]
    fn bow_length_example_2() {
        let length = bow_length(1, 1, 10);
        assert_eq!(10, length)
    }

    #[test]
    fn ribbon_wrap_example_1() {
        let ribbon_wrap_length = ribbon_wrap_length(2, 3, 4);
        assert_eq!(10, ribbon_wrap_length)
    }

    #[test]
    fn ribbon_wrap_example_2() {
        let ribbon_wrap_length = ribbon_wrap_length(1, 1, 10);
        assert_eq!(4, ribbon_wrap_length)
    }

    #[test]
    fn total_ribbon_example_1() {
        let present_box = PresentBox {
            length: 2,
            width: 3,
            height: 4,
        };
        assert_eq!(34, calculate_needed_ribbon(present_box))
    }

    #[test]
    fn total_ribbon_example_2() {
        let present_box = PresentBox {
            length: 1,
            width: 1,
            height: 10,
        };
        assert_eq!(14, calculate_needed_ribbon(present_box))
    }
}
