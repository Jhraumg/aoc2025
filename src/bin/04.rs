use std::collections::HashSet;

advent_of_code::solution!(4);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}
fn count_adjacent_rolls(p: &Point, rolls: &HashSet<Point>) -> usize {
    let minx = if p.x == 0 { 0 } else { p.x - 1 };
    let miny = if p.y == 0 { 0 } else { p.y - 1 };
    (minx..p.x + 2)
        .flat_map(move |x| (miny..p.y + 2).map(move |y| (x, y)))
        .filter(|(x, y)| (*x != p.x || *y != p.y) && rolls.contains(&Point { x: *x, y: *y }))
        .count()
}

fn read_rolls(input: &str) -> HashSet<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().filter_map(
                move |(x, c)| {
                    if c == '@' { Some(Point { x, y }) } else { None }
                },
            )
        })
        .collect()
}
pub fn part_one(input: &str) -> Option<u64> {
    let rolls = read_rolls(input);
    Some(
        rolls
            .iter()
            .filter(|p| count_adjacent_rolls(p, &rolls) < 4)
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut rolls = read_rolls(input);
    let mut all_moved_count = 0;
    loop {
        let moved: HashSet<Point> = rolls
            .iter()
            .filter(|p| count_adjacent_rolls(p, &rolls) < 4)
            .copied()
            .collect();
        if moved.is_empty() {
            break;
        }
        all_moved_count += moved.len() as u64;
        // rolls = rolls.difference(&moved).copied().collect();
        for m in &moved {
            rolls.remove(m);
        }
    }

    Some(all_moved_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
