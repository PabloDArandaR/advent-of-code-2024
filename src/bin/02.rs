advent_of_code::solution!(2);

const MAX_DIFF: u32 = 3;
const MIN_DIFF: u32 = 1;

pub fn correct_difference(val1: u32, val2: u32) -> bool {
    if (val1.abs_diff(val2) <= MAX_DIFF) && (val1.abs_diff(val2) >= MIN_DIFF) {
        return true;
    }
    return false;
}

pub fn is_series_decreasing(val1: u32, val2: u32) -> bool {
    return val1 > val2;
}

pub fn part_one(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.split('\n').collect();
    let mut n_safe: u32 = 0;

    for report in splitted_input {
        if report.len() <= 1 {
            continue;
        }
        let splitted_report: Vec<u32> = report
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let mut is_safe: bool = true;
        let is_decreasing: bool = is_series_decreasing(splitted_report[0], splitted_report[1]);
        for i in 0..(splitted_report.len() - 1) {
            if (is_series_decreasing(splitted_report[i], splitted_report[i + 1]) != is_decreasing)
                || !correct_difference(splitted_report[i], splitted_report[i + 1])
            {
                is_safe = false;
            }
        }
        if is_safe {
            n_safe += 1;
        }
    }

    return Some(n_safe);
}

pub fn part_two(input: &str) -> Option<u32> {
    let splitted_input: Vec<&str> = input.split('\n').collect();
    let mut n_safe: u32 = 0;

    for report in splitted_input {
        if report.len() <= 1 {
            continue;
        }
        let splitted_report: Vec<u32> = report
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let mut is_safe: bool = true;
        let mut is_decreasing: bool = is_series_decreasing(splitted_report[0], splitted_report[1]);
        let mut already_skipped_one: bool = false;
        let mut skipped_index = splitted_report.len() + 1;
        for i in 1..(splitted_report.len()) {
            let mut first_index = i - 1;
            if first_index == skipped_index {
                first_index -= 1;
            }
            if (is_series_decreasing(splitted_report[first_index], splitted_report[i])
                != is_decreasing)
                || !correct_difference(splitted_report[first_index], splitted_report[i])
            {
                if already_skipped_one {
                    is_safe = false;
                    break;
                } else {
                    if i == 1 {
                        if correct_difference(splitted_report[0], splitted_report[2])
                            && correct_difference(splitted_report[1], splitted_report[2])
                        {
                            if is_series_decreasing(splitted_report[1], splitted_report[2])
                                == is_series_decreasing(splitted_report[2], splitted_report[3])
                            {
                                is_decreasing =
                                    is_series_decreasing(splitted_report[1], splitted_report[2]);
                                skipped_index = 0;
                                already_skipped_one = true;
                            } else {
                                is_decreasing =
                                    is_series_decreasing(splitted_report[0], splitted_report[2]);
                                skipped_index = 1;
                                already_skipped_one = true;
                            }
                        } else if correct_difference(splitted_report[0], splitted_report[2]) {
                            is_decreasing =
                                is_series_decreasing(splitted_report[0], splitted_report[2]);
                            skipped_index = 1;
                            already_skipped_one = true;
                        } else if correct_difference(splitted_report[1], splitted_report[2]) {
                            is_decreasing =
                                is_series_decreasing(splitted_report[1], splitted_report[2]);
                            skipped_index = 0;
                            already_skipped_one = true;
                        } else {
                            is_safe = false;
                            break;
                        }
                    } else if i + 1 >= splitted_report.len() {
                        break;
                    } else if correct_difference(
                        splitted_report[first_index],
                        splitted_report[i + 1],
                    ) && is_series_decreasing(
                        splitted_report[first_index],
                        splitted_report[i + 1],
                    ) == is_decreasing
                    {
                        skipped_index = i;
                        already_skipped_one = true;
                    } else if correct_difference(
                        splitted_report[first_index - 1],
                        splitted_report[i],
                    ) && is_series_decreasing(
                        splitted_report[first_index - 1],
                        splitted_report[i],
                    ) == is_decreasing
                    {
                        skipped_index = first_index;
                        already_skipped_one = true;
                    } else {
                        is_safe = false;
                        break;
                    }
                }
            }
        }
        if is_safe {
            n_safe += 1;
        }
    }
    return Some(n_safe);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
