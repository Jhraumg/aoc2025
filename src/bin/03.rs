use std::cmp::max;

advent_of_code::solution!(3);

fn get_bank_voltage(bank: &str) -> u64 {
    let volts: (Option<u64>, Option<u64>) =
        bank.chars()
            .filter_map(|c| c.to_digit(10))
            .fold((None, None), |acc, d| {
                let d = d as u64;
                match acc {
                    (None, None) => (Some(d), None),
                    (Some(first), None) => (Some(first), Some(d)),
                    (Some(first), Some(second)) => {
                        let r1 = 10 * first + second;
                        let r2 = 10 * first + d;
                        let r3 = 10 * second + d;
                        match max(max(r1, r2), r3) {
                            v if v == r1 => (Some(first), Some(second)),
                            v if v == r2 => (Some(first), Some(d)),
                            v if v == r3 => (Some(second), Some(d)),
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                }
            });
    volts.0.unwrap() * 10 + volts.1.unwrap()
}

fn joltage(batteries: &[u32]) -> u64 {
    batteries.iter().fold(0, |acc, b| acc * 10 + *b as u64)
}
fn get_bank_voltage_n(bank: &str, size: usize) -> u64 {
    let volts: Vec<u32> = bank
        .chars()
        .filter_map(|c| c.to_digit(10))
        .fold(vec![], |mut acc, d| {
            if acc.len() < size {
                acc.push(d);
                return acc;
            }

            let mut max_joltage = joltage(&acc);
            let mut max_acc = acc.clone();

            for i in 0..size {
                let mut new_acc: Vec<u32> = acc
                    .iter()
                    .enumerate()
                    .filter_map(|(j, c)| if i != j { Some(*c) } else { None })
                    .collect();
                new_acc.push(d);

                let new_joltage = joltage(&new_acc);
                if new_joltage > max_joltage {
                    max_joltage = new_joltage;
                    max_acc = new_acc;
                }
            }
            max_acc
        });

    assert_eq!(volts.len(), 12);
    joltage(&volts)
}
pub fn part_one(input: &str) -> Option<u64> {
    Some(input.trim().lines().map(get_bank_voltage).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .lines()
            .map(|bank| get_bank_voltage_n(bank, 12))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
