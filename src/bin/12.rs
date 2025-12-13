use itertools::Itertools;
use std::num::TryFromIntError;
use std::str::FromStr;

advent_of_code::solution!(12);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Point {
    x: u8,
    y: u8,
}

#[non_exhaustive]
#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
struct Present {
    occupied: Vec<Point>, // always sorted, to keep Hash relevant, hence the non exhaustive
    maxx: u8,
    maxy: u8,
}

impl Present {
    fn new(points: &[Point]) -> Self {
        let occupied = points.iter().cloned().sorted().collect();
        let maxx = points.iter().map(|Point { x, y: _ }| *x).max().unwrap();
        let maxy = points.iter().map(|Point { x: _, y }| *y).max().unwrap();
        Self {
            occupied,
            maxx,
            maxy,
        }
    }
    fn rotate_left(&self) -> Self {
        let occupied: Vec<Point> = self
            .occupied
            .iter()
            .map(|Point { x, y }| Point {
                x: self.maxy - *y,
                y: *x,
            })
            .sorted()
            .collect();
        let maxx = self.maxy;
        let maxy = self.maxx;
        Self {
            occupied,
            maxx,
            maxy,
        }
    }

    fn flip_horizontal(&self) -> Self {
        let occupied = self
            .occupied
            .iter()
            .map(|Point { x, y }| Point {
                x: self.maxx - x,
                y: *y,
            })
            .sorted()
            .collect();
        Self {
            occupied,
            maxx: self.maxx,
            maxy: self.maxy,
        }
    }
    fn all_presentations(&self) -> Vec<Present> {
        [self.clone(), self.flip_horizontal()]
            .into_iter()
            .flat_map(|shape| {
                [
                    shape.rotate_left(),
                    shape.rotate_left().rotate_left(),
                    shape.rotate_left().rotate_left().rotate_left(),
                    shape,
                ]
                .into_iter()
            })
            .unique()
            .collect()
    }
}

#[derive(Debug)]
struct Tree {
    shape: Point,
    presents: Vec<usize>,
}
struct Cavern {
    presents: Vec<Present>,
    trees: Vec<Tree>,
}

impl FromStr for Cavern {
    type Err = TryFromIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut presents = Vec::new();
        let mut current_y = 0;
        let mut points = Vec::new();
        for l in s.lines() {
            if l.contains("x") {
                break;
            }
            if l.is_empty() {
                presents.push(Present::new(&points));
                points.clear();
            }
            if l.contains(':') {
                current_y = 0;
            }
            for (i, _) in l.chars().enumerate().filter(|(_, c)| *c == '#') {
                points.push(Point {
                    x: i.try_into()?,
                    y: current_y - 1,
                });
            }
            current_y += 1;
        }

        let mut trees = Vec::new();
        for l in s.lines().skip_while(|l| !l.contains('x')) {
            let (shape, presents) = l.split_once(':').unwrap();
            let (x, y) = shape
                .split('x')
                .map(|w| w.trim().parse::<u8>().unwrap())
                .collect_tuple()
                .unwrap();
            let presents = presents
                .split_whitespace()
                .map(|w| w.parse::<usize>().unwrap())
                .collect();
            trees.push(Tree {
                shape: Point { x, y },
                presents,
            });
        }

        Ok(Self { presents, trees })
    }
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
struct TreeSpace {
    occupied: Vec<bool>, // must be always sorted
    shape: Point,
}
impl TreeSpace {
    fn new(shape: Point) -> Self {
        Self {
            occupied: vec![false; shape.x as usize * shape.y as usize],
            shape,
        }
    }
    fn prev_pos(&self, current_pos: Point) -> Option<Point> {
        if current_pos.x > 0 {
            Some(Point {
                x: current_pos.x - 1,
                y: current_pos.y,
            })
        } else if current_pos.y > 0 {
            Some(Point {
                x: self.shape.x - 1,
                y: current_pos.y - 1,
            })
        } else {
            None
        }
    }
    fn next_pos(&self, current_pos: Point) -> Option<Point> {
        if current_pos.x + 1 < self.shape.x {
            Some(Point {
                x: current_pos.x + 1,
                y: current_pos.y,
            })
        } else if current_pos.y + 1 < self.shape.y {
            Some(Point {
                x: 0,
                y: current_pos.y + 1,
            })
        } else {
            None
        }
    }

    fn occupied(&self, pos: &Point) -> bool {
        self.occupied[pos.y as usize * self.shape.x as usize + pos.x as usize]
    }
    fn insert(&mut self, pos: &Point) {
        debug_assert!(!self.occupied(pos));
        self.occupied[pos.y as usize * self.shape.x as usize + pos.x as usize] = true;
    }
    fn remove(&mut self, pos: &Point) {
        self.occupied[pos.y as usize * self.shape.x as usize + pos.x as usize] = false;
    }

    fn place_present(&mut self, present: &Present, pos: &Point) -> Result<(), ()> {
        if present.occupied.iter().any(|p| {
            p.x + pos.x >= self.shape.x
                || p.y + pos.y >= self.shape.y
                || self.occupied(&Point {
                    x: p.x + pos.x,
                    y: p.y + pos.y,
                })
        }) {
            Err(())
        } else {
            for p in &present.occupied {
                self.insert(&Point {
                    x: p.x + pos.x,
                    y: p.y + pos.y,
                });
            }
            Ok(())
        }
    }

    fn remove_present(&mut self, present: &Present, pos: &Point) {
        for p in &present.occupied {
            self.remove(&Point {
                x: p.x + pos.x,
                y: p.y + pos.y,
            });
        }
    }
}

impl Cavern {
    fn next_available_shape(
        current_shape: &Option<(u8, u8)>,
        missings: &[usize],
        variant_counts: &[usize],
    ) -> Option<(u8, u8)> {
        if let Some((cur_p, cur_v)) = current_shape {
            if (*cur_v as usize) < (variant_counts[*cur_p as usize] - 1) {
                Some((*cur_p, *cur_v + 1))
            } else {
                missings
                    .iter()
                    .enumerate()
                    .skip(*cur_p as usize + 1)
                    .find(|(_, m)| **m > 0)
                    .and_then(|(i, _)| i.try_into().ok().map(|i| (i, 0u8)))
            }
        } else {
            missings
                .iter()
                .enumerate()
                .find(|(_, m)| **m > 0)
                .and_then(|(i, _)| i.try_into().ok().map(|i| (i, 0u8)))
        }
    }
    fn presents_fit_under_tree(&self, tree: &Tree, all_shapes: &[Vec<Present>]) -> bool {
        let variant_counts: Vec<usize> = all_shapes.iter().map(|v| v.len()).collect();

        let mut missing_presents = tree.presents.clone();

        let mut space = TreeSpace::new(tree.shape);
        // a present is modeled by its coordinates+ its variant as (present_idx, variant_idx)
        let mut options_used: Vec<Option<(u8, u8)>> =
            Vec::with_capacity(tree.shape.x as usize * tree.shape.y as usize);

        let mut current_option: Option<(u8, u8)> = None;
        let mut current_pos: Point = Point { x: 0, y: 0 };
        'main: loop {
            if missing_presents.iter().all(|m| *m == 0) {
                return true;
            }
            let empty_places = (tree.shape.y as usize - current_pos.y as usize - 1)
                * tree.shape.x as usize
                - current_pos.x as usize;
            if empty_places
                > missing_presents
                    .iter()
                    .enumerate()
                    .map(|(pre_idx, count)| all_shapes[pre_idx][0].occupied.len() * *count)
                    .sum()
            {
                // next shape
                current_option =
                    Self::next_available_shape(&current_option, &missing_presents, &variant_counts);
                if let Some((present_idx, variant_idx)) = current_option {
                    if space
                        .place_present(
                            &all_shapes[present_idx as usize][variant_idx as usize],
                            &current_pos,
                        )
                        .is_ok()
                    {
                        assert!(missing_presents[present_idx as usize] > 0);
                        missing_presents[present_idx as usize] -= 1;
                    } else {
                        // let's try next_shape
                        continue;
                    }
                }
                //  moving to the next point, all options tested here
                if let Some(new_pos) = space.next_pos(current_pos) {
                    options_used.push(current_option);
                    current_pos = new_pos;
                    current_option = None;
                    continue;
                }
            }
            //backtrack
            while !options_used.is_empty() {
                current_pos = space.prev_pos(current_pos).unwrap();
                let last_pres = options_used.pop().unwrap();

                if let Some((prev_pres, prev_var)) = last_pres {
                    //let's try the shape after the one used in the previous attempt
                    space.remove_present(
                        &all_shapes[prev_pres as usize][prev_var as usize],
                        &current_pos,
                    );
                    missing_presents[prev_pres as usize] += 1;
                    current_option = Some((prev_pres, prev_var));

                    continue 'main;
                }
            }

            return false;
        }
    }
    fn presents_fit_count(&self) -> usize {
        let all_shapes = self
            .presents
            .iter()
            .map(|p| p.all_presentations())
            .collect_vec();

        self.trees
            .iter()
            .filter(|tree| self.presents_fit_under_tree(tree, &all_shapes))
            .count()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let cavern = Cavern::from_str(input).unwrap();
    Some(cavern.presents_fit_count() as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
