advent_of_code::solution!(1);

pub fn clean_pair(input: &str) -> (u32, u32) {
    let splitted: Vec<&str> = input.split(' ').collect();
    return (
        splitted[0].parse::<u32>().unwrap(),
        splitted[splitted.len() - 1].parse::<u32>().unwrap(),
    );
}

pub fn strip_str(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();
    let splitted_input: Vec<&str> = input.split('\n').collect();
    for c in splitted_input {
        if c.len() < 1 {
            continue;
        }
        let (left, right) = clean_pair(c);
        left_list.push(left);
        right_list.push(right);
    }

    return (left_list, right_list);
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_list, mut right_list) = strip_str(input);
    left_list.sort();
    right_list.sort();
    let mut difference: u32 = 0;
    for n in 0..left_list.len() {
        let new_val = left_list[n].abs_diff(right_list[n]);
        difference += new_val;
    }
    return Some(difference);
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut left_list, mut right_list) = strip_str(input);
    left_list.sort();
    right_list.sort();
    let mut similarity_score: u32 = 0;
    let mut index: usize = 0;
    let max_index = left_list.len();
    for val in left_list {
        let mut n_occurrences: usize = 0;
        while (right_list[index] < val) {
            index += 1;
            if (index >= max_index) {
                break;
            }
        }

        if (index >= max_index) {
            break;
        }
        while (right_list[index] == val) {
            n_occurrences += 1;

            index += 1;
            if (index >= max_index) {
                break;
            }
        }
        similarity_score += val * n_occurrences as u32;
    }
    return Some(similarity_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
