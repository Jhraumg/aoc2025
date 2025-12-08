use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

advent_of_code::solution!(8);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}
impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s
            .trim()
            .split(',')
            .map(|w| w.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Point { x, y, z })
    }
}

impl Point {
    fn square_distance(&self, p: &Point) -> u64 {
        let result = (p.x - self.x) * (p.x - self.x)
            + (p.y - self.y) * (p.y - self.y)
            + (p.z - self.z) * (p.z - self.z);
        result as u64
    }
}

fn connect_boxes(input: &str, count: usize) -> usize {
    let points: Vec<Point> = input.lines().map(|l| l.parse().unwrap()).collect();

    let known_distances: HashMap<(Point, Point), u64> = points
        .iter()
        .flat_map(|p1| points.iter().map(move |p2| (p1, p2)))
        .filter_map(|(p1, p2)| {
            if p1 != p2 {
                Some(((min(*p1, *p2), max(*p1, *p2)), p1.square_distance(p2)))
            } else {
                None
            }
        })
        .collect();

    let mut connected: HashSet<Point> = HashSet::new();
    let mut connection_set: Vec<HashSet<Point>> = vec![];
    for ((p1, p2), _) in known_distances
        .iter()
        .sorted_by(|(_, d1), (_, d2)| d1.cmp(d2))
        .take(count)
    {
        if connected.contains(p1) {
            if connected.contains(p2) {
                let merged_set: HashSet<Point> = connection_set
                    .iter()
                    .find(|set| set.contains(p1))
                    .unwrap()
                    .union(connection_set.iter().find(|set| set.contains(p2)).unwrap())
                    .copied()
                    .collect();

                let mut new_set: Vec<_> = connection_set
                    .into_iter()
                    .filter(|set| !(set.contains(p1) || set.contains(p2)))
                    .collect();
                new_set.push(merged_set);
                connection_set = new_set;
            } else {
                connection_set = connection_set
                    .into_iter()
                    .map(|set| {
                        if set.contains(p1) {
                            let mut new_set = set;
                            new_set.insert(*p2);
                            new_set
                        } else {
                            set
                        }
                    })
                    .collect();
            }
        } else if connected.contains(p2) {
            connection_set = connection_set
                .into_iter()
                .map(|set| {
                    if set.contains(p2) {
                        let mut new_set = set;
                        new_set.insert(*p1);
                        new_set
                    } else {
                        set
                    }
                })
                .collect();
        } else {
            connection_set.push(HashSet::from_iter([*p1, *p2].into_iter()));
        }

        connected.insert(*p1);
        connected.insert(*p2);
    }

    connection_set
        .iter()
        .map(|set| set.len())
        .sorted_by(|s1, s2| s1.cmp(s2))
        .rev()
        .take(3)
        .product()
}

fn connect_last(input: &str) -> u64 {
    let points: Vec<Point> = input.lines().map(|l| l.parse().unwrap()).collect();

    let known_distances: HashMap<(Point, Point), u64> = points
        .iter()
        .flat_map(|p1| points.iter().map(move |p2| (p1, p2)))
        .filter_map(|(p1, p2)| {
            if p1 != p2 {
                Some(((min(*p1, *p2), max(*p1, *p2)), p1.square_distance(p2)))
            } else {
                None
            }
        })
        .collect();

    let mut connected: HashSet<Point> = HashSet::new();
    let mut connection_set: Vec<HashSet<Point>> = vec![];

    for ((p1, p2), _) in known_distances
        .iter()
        .sorted_by(|(_, d1), (_, d2)| d1.cmp(d2))
    {
        if connected.contains(p1) {
            if connected.contains(p2) {
                let merged_set: HashSet<Point> = connection_set
                    .iter()
                    .find(|set| set.contains(p1))
                    .unwrap()
                    .union(connection_set.iter().find(|set| set.contains(p2)).unwrap())
                    .copied()
                    .collect();

                let mut new_set: Vec<_> = connection_set
                    .into_iter()
                    .filter(|set| !(set.contains(p1) || set.contains(p2)))
                    .collect();
                new_set.push(merged_set);
                connection_set = new_set;
            } else {
                connection_set = connection_set
                    .into_iter()
                    .map(|set| {
                        if set.contains(p1) {
                            let mut new_set = set;
                            new_set.insert(*p2);
                            new_set
                        } else {
                            set
                        }
                    })
                    .collect();
            }
        } else if connected.contains(p2) {
            connection_set = connection_set
                .into_iter()
                .map(|set| {
                    if set.contains(p2) {
                        let mut new_set = set;
                        new_set.insert(*p1);
                        new_set
                    } else {
                        set
                    }
                })
                .collect();
        } else {
            connection_set.push(HashSet::from_iter([*p1, *p2].into_iter()));
        }

        connected.insert(*p1);
        connected.insert(*p2);
        if connected.len() == points.len() && connection_set.len() == 1 {
            return (p1.x * p2.x) as u64;
        }
    }

    unreachable!()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(connect_boxes(input, 1000) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(connect_last(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = connect_boxes(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part_two() {
        let result = connect_last(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 25272);
    }
}
