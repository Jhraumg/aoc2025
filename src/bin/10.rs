use itertools::Itertools;
use std::cmp::min;
use std::str::FromStr;

#[cfg(feature = "z3")]
use z3::{Optimize, ast::Int};

#[cfg(feature = "z3")]
use std::collections::HashMap;

advent_of_code::solution!(10);

#[derive(Debug)]
struct FactoryLine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}
impl FromStr for FactoryLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lights_end = s.find(']').unwrap();
        let lights = s[1..lights_end]
            .chars()
            .map(|c| match c {
                '#' => true,
                '.' => false,
                _ => unreachable!("invalid lights status '{c}' in '{s}'"),
            })
            .collect();
        let joltage_start = s.find('{').unwrap();
        let buttons: Vec<_> = s[lights_end + 1..joltage_start]
            .split_ascii_whitespace()
            .map(|w| {
                w[1..w.len() - 1]
                    .split(',')
                    .map(|b| b.parse::<usize>().unwrap())
                    .collect_vec()
            })
            .collect();
        let joltages: Vec<usize> = s[joltage_start + 1..s.len() - 1]
            .split(',')
            .map(|j| j.trim().parse().unwrap())
            .collect();

        // let's reorder buttons by max number of presses for joltages
        let max_number_of_press_by_joltages = |button: &[usize]| {
            button
                .iter()
                .map(|jidx| joltages[*jidx])
                .reduce(min)
                .unwrap()
        };

        let buttons: Vec<_> = buttons
            .into_iter()
            .sorted_by(|b1, b2| {
                max_number_of_press_by_joltages(b1).cmp(&max_number_of_press_by_joltages(b2))
            })
            .collect();
        // println!("buttons max pres : {}", buttons.iter().map(|b|max_number_of_press_by_joltages(b)).join(","));
        Ok(FactoryLine {
            lights,
            buttons,
            joltages,
        })
    }
}
impl FactoryLine {
    fn min_number_of_press(&self) -> usize {
        let max_len = self.buttons.len();
        // at most one press
        (0..=max_len)
            .find({
                |k| {
                    let k = *k;
                    (0..max_len).combinations(k).any(|buttons| {
                        let mut lights_status = vec![false; self.lights.len()];
                        for b_idx in buttons {
                            for light_idx in &self.buttons[b_idx] {
                                lights_status[*light_idx] = !lights_status[*light_idx];
                            }
                        }
                        lights_status
                            .iter()
                            .zip(self.lights.iter())
                            .all(|(status, target)| *status == *target)
                    })
                }
            })
            .unwrap()
    }

    #[cfg(not(feature = "z3"))]
    fn min_number_of_press_joltages(&self) -> Option<usize> {
        let _ = self.joltages;
        todo!("disabled because z3 build is too slow. Use cargo run --features=z3 --bin 10  to get the actual solution")
    }
    #[cfg(feature = "z3")]
    fn min_number_of_press_joltages(&self) -> Option<usize> {
        // buttons_var=list(Int(f"b{i}") for i in range(len(buttons)))
        let buttons: Vec<Int> = (0..self.buttons.len())
            .map(|i| Int::new_const(format!("b{i}")))
            .collect();
        let buttons_by_joltage: HashMap<usize, Vec<usize>> =
            self.buttons
                .iter()
                .enumerate()
                .fold(HashMap::new(), |mut acc, (b_idx, joltages)| {
                    for j_idx in joltages {
                        acc.entry(*j_idx)
                            .and_modify(|buttons| buttons.push(b_idx))
                            .or_insert(vec![b_idx]);
                    }
                    acc
                });

        let opt = Optimize::new();
        for b in &buttons {
            opt.assert(&b.ge(0));
        }

        for (joltage, buttons_idx) in &buttons_by_joltage {
            opt.assert(
                &buttons_idx
                    .iter()
                    .map(|b_idx| &buttons[*b_idx])
                    .cloned()
                    .reduce(|acc, b| acc + b)
                    .unwrap()
                    .eq(self.joltages[*joltage] as u64),
            );
        }
        let cost: Int = buttons.iter().cloned().reduce(|acc, b| acc + b).unwrap();
        opt.minimize(&cost);
        if let z3::SatResult::Sat = opt.check(&[]) {
            opt.get_model()?
                .eval(&cost, true)
                .and_then(|c| c.as_u64())
                .map(|c| c as usize)
        } else {
            None
        }
    }
}
struct Factory {
    lines: Vec<FactoryLine>,
}
impl FromStr for Factory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Result<Vec<FactoryLine>, _> =
            s.lines().map(|l| l.parse::<FactoryLine>()).collect();
        Ok(Factory { lines: lines? })
    }
}
impl Factory {
    fn min_number_of_press(&self) -> usize {
        self.lines.iter().map(|l| l.min_number_of_press()).sum()
    }

    fn min_number_of_press_joltages(&self) -> usize {
        self.lines
            .iter()
            .map(|l| l.min_number_of_press_joltages().unwrap())
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let factory = Factory::from_str(input).unwrap();
    Some(factory.min_number_of_press() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let factory = Factory::from_str(input).unwrap();
    Some(factory.min_number_of_press_joltages() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let line: FactoryLine = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
            .parse()
            .unwrap();
        assert_eq!(line.min_number_of_press(), 2);

        let line: FactoryLine = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"
            .parse()
            .unwrap();
        assert_eq!(line.min_number_of_press(), 3);

        let line: FactoryLine = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
            .parse()
            .unwrap();
        assert_eq!(line.min_number_of_press(), 2);

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
