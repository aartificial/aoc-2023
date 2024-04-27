use itertools::Itertools;

pub struct Sequence<'a> {
    pub numbers: &'a [i32],
    pub diffs: Vec<Vec<i32>>,
}

impl Sequence<'_> {
    pub(crate) fn parse(input: &str) -> Vec<Vec<i32>> {
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect_vec()
            })
            .collect_vec()
    }

    pub(crate) fn process(sequence: &[i32]) -> Vec<Vec<i32>> {
        let mut s: Vec<Vec<i32>> = vec![sequence.to_vec(), Self::diff(sequence)];

        while Self::all_zero(&s) {
            let last_diff = s.last().unwrap();
            s.push(Self::diff(last_diff));
        }

        s
    }
    fn diff(sequence: &[i32]) -> Vec<i32> {
        sequence
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect_vec()
    }
    fn all_zero(s: &[Vec<i32>]) -> bool {
        s.last().unwrap().iter().any(|&x| x != 0)
    }
}
