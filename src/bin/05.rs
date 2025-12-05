use itertools::Itertools;
use std::str::FromStr;

advent_of_code::solution!(5);

#[derive(Debug)]
struct FreshList {
    list: Vec<(u64, u64)>,
}

impl FromStr for FreshList {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let list = s
            .lines()
            .take_while(|l| !l.is_empty())
            .map(|l| {
                l.split('-')
                    .filter_map(|w| w.trim().parse::<u64>().ok())
                    .collect_tuple()
                    .unwrap()
            })
            .sorted_by(|(x1, _), (x2, _)| x1.cmp(x2))
            .collect();
        Ok(Self { list })
    }
}
impl FreshList {
    fn is_fresh(&self, ingredient: u64) -> bool {
        for (imin, imax) in &self.list {
            if *imin > ingredient {
                return false;
            }
            if *imax >= ingredient {
                // println!("{ingredient} is fresh");
                return true;
            }
        }
        false
    }
    fn count_fresh(&self) -> u64 {
        self.list
            .iter()
            .fold((0, 0), |(count, current_max), (ing_min, ing_max)| {
                if count == 0 {
                    (*ing_max - *ing_min + 1, *ing_max)
                } else if current_max < *ing_min {
                    (count + *ing_max - *ing_min + 1, *ing_max)
                } else if current_max >= *ing_max {
                    (count, current_max)
                } else {
                    // dont' count current_max twice !
                    (count + *ing_max - current_max, *ing_max)
                }
            })
            .0
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let fresshes = FreshList::from_str(input).unwrap();
    // println!("{fresshes:?}");
    Some(
        input
            .lines()
            .skip(fresshes.list.len())
            .filter_map(|l| l.parse::<u64>().ok())
            .filter(|f| fresshes.is_fresh(*f))
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let fresshes = FreshList::from_str(input).unwrap();
    Some(fresshes.count_fresh())
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
        assert_eq!(result, Some(14));
    }
}
