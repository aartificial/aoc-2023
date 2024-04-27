use crate::custom_error::AocError;
use itertools::Itertools;
use rayon::prelude::*;
use std::ops::Range;
use tracing::info;

struct Map {
    sd: Vec<(Range<u64>, Range<u64>)>,
}

impl Map {
    fn from_categories(mut sd: Vec<(Range<u64>, Range<u64>)>) -> Self {
        sd.sort_by_key(|(s, _)| s.start);
        Self { sd }
    }

    fn get(&self, key: u64) -> u64 {
        let index = self.sd.partition_point(|(s, _)| s.start <= key) as i32 - 1;

        if index < 0 {
            return key;
        }

        self.sd
            .get(index as usize)
            .and_then(|(s, d)| (key <= s.end).then_some(d.start + (key.saturating_sub(s.start))))
            .unwrap_or(key)
    }
}

fn parse_maps(s: &str) -> Vec<Map> {
    s.trim()
        .split("\n\n")
        .map(|m| {
            Map::from_categories(
                m.lines()
                    .skip(1)
                    .map(|sd| {
                        let sd_map = sd.split_whitespace();
                        let sd_n = sd_map.take(3);
                        let sd_n = sd_n.map(|x| x.parse::<u64>().unwrap());
                        let [d, s, n]: [u64; 3] = sd_n.collect_vec().try_into().unwrap();
                        (s..s + n, d..d + n)
                    })
                    .collect(),
            )
        })
        .collect()
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (seeds, maps) = input.split_once("\n\n").unwrap();

    let seeds = seeds.strip_prefix("seeds: ").unwrap();
    let seeds: Vec<u64> = seeds
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let maps = parse_maps(maps);

    let ranges = seeds
        .chunks_exact(2)
        .map(|range| {
            let (start, distance) = (range[0], range[1]);
            let seeds = start..start + distance;
            seeds.map(|seed| maps.iter().fold(seed, |acc, map| map.get(acc)))
        })
        .collect_vec();

    info!("ranges: {:?}", ranges);

    let min = ranges
        .into_par_iter()
        .inspect(|range| info!("range: {:?}", range))
        .map(|range| range.min().unwrap())
        .inspect(|min| info!("min: {}", min))
        .min()
        .unwrap();

    Ok(min)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::fmt;

    #[test]
    fn test_process() -> miette::Result<()> {
        fmt::init();
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(46, process(input)?);
        Ok(())
    }
}
