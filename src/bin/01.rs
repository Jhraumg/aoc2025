advent_of_code::solution!(1);

fn count_zero_stops(input: &str) -> u64 {
    let mut count = 0u64;
    let mut val = 50;
    for line in input.lines() {
        let rotate: i64 =
            if line.starts_with('L') { -1 } else { 1 } * line[1..].parse::<i64>().unwrap();
        let new_val = val + rotate;

        val = ((new_val % 100) + 100) % 100;
        if val == 0 {
            count += 1
        };
    }

    count
}

fn count_zero_pass(input: &str) -> u64 {
    let mut count = 0u64;
    let mut val = 50;
    for line in input.lines() {
        let rotate: i64 =
            if line.starts_with('L') { -1 } else { 1 } * line[1..].parse::<i64>().unwrap();
        let new_val = val + rotate;

        let delta = (new_val - val).abs();
        count += delta as u64 / 100;

        let remain = (delta % 100) * rotate.signum() + val;
        if val != 0 && (remain <= 0 || remain >= 100) {
            count += 1;
        }

        val = ((new_val % 100) + 100) % 100;
    }

    count
}
pub fn part_one(input: &str) -> Option<u64> {
    Some(count_zero_stops(input))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(count_zero_pass(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
