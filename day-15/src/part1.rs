use crate::custom_error::AocError;

pub fn hash(input: &str) -> usize {
    let mut acc = 0;
    for byte in input.bytes() {
        acc = (acc + byte as usize) * 17 % 256;
    }
    acc
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    Ok(input.split(',').map(|s| s.trim()).map(hash).sum())
}

#[cfg(test)]
mod tests {
    use crate::part1::hash;
    use rstest::rstest;

    #[rstest]
    #[case("H", 200)]
    #[case("HA", 153)]
    #[case("HAS", 172)]
    #[case("HASH", 52)]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn hash_test(#[case] input: &str, #[case] expected: usize) {
        let result = hash(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn sum_hash_test() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result: usize = input.split(',').map(|s| s.trim()).map(hash).sum();
        assert_eq!(1320, result);
    }
}
