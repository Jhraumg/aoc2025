use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;

advent_of_code::solution!(9);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Point {
    x: u64,
    y: u64,
}
pub fn part_one(input: &str) -> Option<u64> {
    let red_tiles: Vec<_> = input
        .trim()
        .lines()
        .map(|l| {
            let (x, y) = l
                .trim()
                .split(',')
                .map(|w| w.parse().unwrap())
                .collect_tuple()
                .unwrap();
            Point { x, y }
        })
        .collect();
    red_tiles
        .iter()
        .filter_map(|p| {
            red_tiles
                .iter()
                .map(|p2| (p2.x.abs_diff(p.x) + 1) * (p2.y.abs_diff(p.y) + 1))
                .reduce(max)
        })
        .reduce(max)
}

fn is_rect_redgreen_only(rect: (Point, Point), vert_edges: &[(u64, (u64, u64))]) -> bool {
    let upperleft = Point {
        x: min(2 * rect.0.x, 2 * rect.1.x),
        y: min(2 * rect.0.y, 2 * rect.1.y),
    };
    let lowerright = Point {
        x: max(2 * rect.0.x, 2 * rect.1.x),
        y: max(2 * rect.0.y, 2 * rect.1.y),
    };

    let considered_edges: HashSet<(u64, (u64, u64))> = vert_edges
        .iter()
        .map(|(col, (top, bottom))| (2 * *col, (2 * *top, 2 * *bottom)))
        .filter(|(col, (top, bottom))| {
            *col <= lowerright.x
                && ((upperleft.y..=lowerright.y).contains(top)
                    || (upperleft.y..=lowerright.y).contains(bottom)
                    || (*top..=*bottom).contains(&upperleft.y))
        })
        .collect();

    let mut intersting_y: HashSet<_> = considered_edges
        .iter()
        .flat_map(|(_, (top, bottom))| [*top, *bottom].into_iter())
        .filter(|l| *l > upperleft.y && *l < lowerright.y)
        .collect();
    intersting_y.insert(upperleft.y);
    intersting_y.insert(lowerright.y);

    let intersting_y = intersting_y
        .iter()
        .flat_map(|l| [*l, *l + 1].into_iter())
        .filter(|l| *l <= lowerright.y)
        .sorted()
        .collect_vec();

    for y in intersting_y {
        let considered_edges: Vec<(u64, (u64, u64))> = considered_edges
            .iter()
            .filter(|(_, (top, bottom))| (*top..=*bottom).contains(&y))
            .sorted_by(|(col1, _), (col2, _)| col1.cmp(col2))
            .copied()
            .collect();

        assert!(upperleft.x > 0);

        if considered_edges
            .first()
            .filter(|(col, _)| *col <= upperleft.x)
            .is_none()
        {
            return false;
        }
        let (mut previous_col, (mut previous_top, mut previous_bottom)) =
            considered_edges.first().copied().unwrap();
        let mut in_green_surface = true;

        for (col, (top, bottom)) in considered_edges.iter().skip(1) {
            if !in_green_surface && *col > previous_col + 1 {
                return false;
            }

            // if we're not on a horizontal edge, extended in the same direction,
            // then we're crossing a boundary, thus switching mode
            if previous_top != *bottom && previous_bottom != *top {
                in_green_surface = !in_green_surface;
            }

            previous_col = *col;
            previous_top = *top;
            previous_bottom = *bottom;
        }
        if !in_green_surface && previous_col != lowerright.x {
            return false;
        }
    }

    true
}
pub fn part_two(input: &str) -> Option<u64> {
    let red_tiles: Vec<_> = input
        .trim()
        .lines()
        .map(|l| {
            let (x, y) = l
                .trim()
                .split(',')
                .map(|w| w.parse().unwrap())
                .collect_tuple()
                .unwrap();
            Point { x, y }
        })
        .collect();
    let len = red_tiles.len();
    assert_eq!(len, red_tiles.iter().unique().count());
    let vert_edges: Vec<(u64, (u64, u64))> = red_tiles
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            let next_p = red_tiles[(i + 1) % len];
            if next_p.x == p.x {
                Some((p.x, (min(p.y, next_p.y), max(p.y, next_p.y))))
            } else {
                None
            }
        })
        .collect();

    red_tiles
        .iter()
        .filter_map(|p| {
            red_tiles
                .iter()
                .filter_map(|p2| {
                    if p2 != p && is_rect_redgreen_only((*p, *p2), &vert_edges) {
                        Some((p2.x.abs_diff(p.x) + 1) * (p2.y.abs_diff(p.y) + 1))
                    } else {
                        None
                    }
                })
                .reduce(max)
        })
        .reduce(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
