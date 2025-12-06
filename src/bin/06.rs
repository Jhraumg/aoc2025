advent_of_code::solution!(6);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

struct Problem<'p> {
    vals: &'p Vec<u64>,
    op: Operation,
}
impl Problem<'_> {
    fn solve(&self) -> u64 {
        match self.op {
            Operation::Add => self.vals.iter().sum(),
            Operation::Multiply => self.vals.iter().product(),
        }
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let mut vals: Vec<Vec<u64>> = vec![];
    let mut opers: Vec<Operation> = vec![];

    for line in input.lines() {
        for (i, w) in line.split_whitespace().enumerate() {
            match w {
                "+" => {
                    opers.push(Operation::Add);
                }
                "*" => {
                    opers.push(Operation::Multiply);
                }
                val => {
                    if i >= vals.len() {
                        vals.push(vec![])
                    };
                    vals[i].push(val.parse().unwrap());
                }
            }
        }
    }

    Some(
        vals.iter()
            .zip(opers.iter())
            .map(|(v, op)| { Problem { vals: v, op: *op } }.solve())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let max_j = input.lines().map(|l| l.len()).max().unwrap();
    let mut vals: Vec<Vec<u64>> = vec![];
    let mut opers: Vec<Operation> = vec![];
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    let mut new_problem = true;
    for i in 0..max_j {
        let i = max_j - i - 1;
        let mut current_val: Option<u64> = None;
        if new_problem {
            vals.push(vec![]);
        }
        // by default, let's consider we're on an empty column
        new_problem = true;
        for line in &lines {
            if line.len() > i {
                match line[i] {
                    c if c.is_ascii_digit() => {
                        new_problem = false;
                        current_val = Some(current_val.unwrap_or(0) * 10 + (c - b'0') as u64);
                    }
                    b'*' => {
                        new_problem = false;
                        opers.push(Operation::Multiply);
                    }
                    b'+' => {
                        new_problem = false;
                        opers.push(Operation::Add);
                    }
                    b' ' => {}
                    c => unreachable!("Unexpected character: {}", c),
                }
            }
        }
        if let Some(val) = current_val {
            vals.last_mut()?.push(val);
        }
    }
    Some(
        vals.iter()
            .zip(opers.iter())
            .map(|(v, op)| { Problem { vals: v, op: *op } }.solve())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
