use std::cmp::{min};
use itertools::Itertools;
use std::str::FromStr;

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
        let joltages :Vec<usize> = s[joltage_start + 1..s.len() - 1]
            .split(',')
            .map(|j| j.trim().parse().unwrap())
            .collect();

        // let's reorder buttons by max number of presses for joltages
        let max_number_of_press_by_joltages= |button : &[usize]| {button.iter().map(|jidx| joltages[*jidx]).reduce(min).unwrap()} ;

        let buttons :Vec<_> = buttons.into_iter().sorted_by(|b1,b2| max_number_of_press_by_joltages(b1).cmp( &max_number_of_press_by_joltages(b2))   ).collect();
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
                    (0..max_len)
                        .combinations(k)
                        .find(|buttons| {
                            let mut lights_status = vec![false; self.lights.len()];
                            for b_idx in buttons {
                                for light_idx in &self.buttons[*b_idx] {
                                    lights_status[*light_idx] = !lights_status[*light_idx];
                                }
                            }
                            lights_status
                                .iter()
                                .zip(self.lights.iter())
                                .all(|(status, target)| *status == *target)
                        })
                        .is_some()
                }
            })
            .unwrap()
    }

    fn max_button_press(&self, current_button: usize, missing_joltages : &[usize]) -> usize {
        self.buttons[current_button].iter().map(|j_idx|missing_joltages[*j_idx]).min().unwrap()
    }
    fn missing_joltages_on_press_nth_button(&self, n: usize, missing_joltages: &[usize]) -> Option<Vec<usize>> {
        if self.buttons[n].iter().any(|i| missing_joltages[*i] ==0) { None } else {
            let mut new_missing = missing_joltages.to_vec();
            for j in &self.buttons[n] {
                new_missing[*j] -= 1;
            }

            Some(new_missing)
        }
    }
    fn _min_number_of_press_joltages(&self, current_button_idx: usize, missing_joltages: &[usize]) -> Option<usize> {
        if missing_joltages.iter().all(|joltage| *joltage == 0) { return Some(0); }
        // println!("current idx{current_button_idx} / {}, missing {:?}",  self.buttons.len(), missing_joltages);
        if current_button_idx == self.buttons.len() {return None;}

        let max_press= self.max_button_press(current_button_idx, missing_joltages);
        (0..=max_press).filter_map(|p| {
            let press = max_press -p ;
            let mut new_missing= missing_joltages.to_vec();
            for idx in &self.buttons[current_button_idx] { new_missing[*idx] -= press; }

            self._min_number_of_press_joltages(current_button_idx+1, &new_missing).map(|count| count + press)
        }).min()
    }

    fn min_number_of_press_joltages(&self) -> usize {
        let Some(result) = self._min_number_of_press_joltages(0,&self.joltages) else{
            unreachable!("No solution for {self:?}");
        };
        print!(".");
        result
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
        self.lines.iter().map(|l| l.min_number_of_press_joltages()).sum()
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
        //
        // let line: FactoryLine = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
        //     .parse()
        //     .unwrap();
        // assert_eq!(line.min_number_of_press_joltages(), 10);
        //
        // let line: FactoryLine = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"
        //     .parse()
        //     .unwrap();
        // assert_eq!(line.min_number_of_press_joltages(), 12);
        //
        // let line: FactoryLine = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
        //     .parse()
        //     .unwrap();
        // assert_eq!(line.min_number_of_press_joltages(), 11);

        // let line: FactoryLine = "[.##.##] (4,5) (0,5) (2,3) (1,3,5) (0,3,5) (0,2,3,5) (0,1,4) (0,2,4,5) {198,181,22,50,173,65}"
        //     .parse()
        //     .unwrap();
        // assert_eq!(line.min_number_of_press_joltages(), 0);


        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
