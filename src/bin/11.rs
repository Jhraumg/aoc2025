use std::collections::{HashMap, HashSet};

advent_of_code::solution!(11);

fn count_path(all_path: &HashMap<&str, HashSet<&str>>, entry: &str, out: &str) -> u64 {
    let mut out_by_entry: HashMap<&str, (usize, Vec<&str>)> = all_path
        .iter()
        .map(|(entry, outs)| {
            (
                *entry,
                (
                    outs.iter().filter(|o| **o == out).count(),
                    outs.iter()
                        .filter(|o| **o != out)
                        .copied()
                        .collect::<Vec<&str>>(),
                ),
            )
        })
        .collect();

    if !out_by_entry.contains_key(entry) {
        println!("no {entry} in {out_by_entry:?}");
    }
    while !out_by_entry[entry].1.is_empty() {
        let new_out_by_entry: HashMap<&str, (usize, Vec<&str>)> = all_path
            .keys()
            .map(|entry| {
                let (count, outs) = &out_by_entry[entry];
                let new_count: usize = count
                    + outs
                        .iter()
                        .map(|e| {
                            if let Some((inner_count, inner_outs)) = out_by_entry.get(e)
                                && inner_outs.is_empty()
                            {
                                *inner_count
                            } else {
                                0
                            }
                        })
                        .sum::<usize>();
                let new_outs = outs
                    .iter()
                    .filter(|e| {
                        !out_by_entry
                            .get(*e)
                            .map(|(_, unresolved)| unresolved.is_empty())
                            .unwrap_or(true)
                    })
                    .cloned()
                    .collect();

                (*entry, (new_count, new_outs))
            })
            .collect();
        out_by_entry = new_out_by_entry;
    }

    out_by_entry[entry].0 as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let path: HashMap<&str, HashSet<&str>> = input
        .lines()
        .map(|l| {
            let entry: &str = &l[..l.find(':').unwrap()];
            let outs: HashSet<&str> = l[l.find(':').unwrap() + 1..]
                .split_ascii_whitespace()
                .collect();

            (entry, outs)
        })
        .collect();

    Some(count_path(&path, "you", "out"))
}

pub fn part_two(input: &str) -> Option<u64> {
    let path: HashMap<&str, HashSet<&str>> = input
        .lines()
        .map(|l| {
            let entry: &str = &l[..l.find(':').unwrap()];
            let outs: HashSet<&str> = l[l.find(':').unwrap() + 1..]
                .split_ascii_whitespace()
                .collect();

            (entry, outs)
        })
        .collect();

    let dac_to_out_count = count_path(&path, "dac", "out");
    let fft_to_out_count = count_path(&path, "fft", "out");
    let dac_to_fft_count = count_path(&path, "dac", "fft");
    let fft_to_dac_count = count_path(&path, "fft", "dac");
    let srv_to_fft_count = count_path(&path, "svr", "fft"); // svr, not srv !
    let srv_to_dac_count = count_path(&path, "svr", "dac");

    Some(
        srv_to_fft_count * fft_to_dac_count * dac_to_out_count
            + srv_to_dac_count * dac_to_fft_count * fft_to_out_count,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out",
        );
        assert_eq!(result, Some(2));
    }
}
