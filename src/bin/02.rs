use itertools::Itertools;

advent_of_code::solution!(2);

fn is_valid_id(id: u64) -> bool {
    let number_of_digit = id.ilog10() + 1;
    if number_of_digit % 2 == 1 {
        return true;
    }
    let split_size = 10u64.pow(number_of_digit >> 1);
    let (high, low) = (id / split_size, id % split_size);
    high != low
}

fn is_valid_id_full(id: u64) -> bool {
    let number_of_digit = id.ilog10() + 1;
    for size in 1..number_of_digit {
        let size = size + 1;
        if !number_of_digit.is_multiple_of(size) {
            continue;
        }
        let split_size = 10u64.pow(number_of_digit / size);
        let pattern = id % split_size;
        let mut remain: u64 = id / split_size;
        while remain > 0 && remain % split_size == pattern {
            remain /= split_size
        }
        // if all fragments were identical, it is a fake id
        if remain == 0 {
            return false;
        };
    }
    true
}

fn sum_invalids(input: &str, is_valid: fn(u64) -> bool) -> u64 {
    input
        .trim()
        .split(',')
        .filter_map(|rg| {
            rg.split('-')
                .map(|d| d.parse::<u64>().unwrap())
                .collect_tuple::<(u64, u64)>()
        })
        .map(|(low, high)| (low..=high).filter(|i| !is_valid(*i)).sum::<u64>())
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(sum_invalids(input, is_valid_id))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(sum_invalids(input, is_valid_id_full))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("11-22"), Some(33));
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        assert!(!is_valid_id_full(565656));
        assert_eq!(part_two("11-22"), Some(33));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
