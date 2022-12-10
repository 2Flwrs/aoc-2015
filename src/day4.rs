use anyhow::{anyhow, Result};
use aoc_runner_derive::aoc;
use md5::{Digest, Md5};

#[aoc(day4, part1)]
fn part1(data: &str) -> Result<usize> {
    let hasher = Md5::new_with_prefix(data);

    for i in 1.. {
        let mut hasher = hasher.clone();

        hasher.update(format!("{i}"));
        let digest = hasher.finalize();

        if digest[0] == 0 && digest[1] == 0 && digest[2] < 0x10 {
            return Ok(i);
        }
    }

    Err(anyhow!("no solution"))
}

#[aoc(day4, part2)]
fn part2(data: &str) -> Result<usize> {
    let hasher = Md5::new_with_prefix(data);

    for i in 1.. {
        let mut hasher = hasher.clone();

        hasher.update(format!("{i}"));
        let digest = hasher.finalize();

        if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
            return Ok(i);
        }
    }

    Err(anyhow!("no solution"))
}

#[cfg(test)]
mod test {
    use super::part1;
    use anyhow::Result;

    #[test]
    #[cfg_attr(
        not(feature = "long-running-tests"),
        ignore = "long running test (use feature 'long-running-tests' to enable)"
    )]
    fn part1_example() -> Result<()> {
        // If your secret key is "abcdef", the answer is 609043, because
        // the MD5 hash of "abcdef609043" starts with five zeroes
        // ("000001dbbfa..."), and it is the lowest such number to do so.
        assert_eq!(part1("abcdef")?, 609043);

        // If your secret key is "pqrstuv", the lowest number it combines
        // with to make an MD5 hash starting with five zeroes is 1048970;
        // that is, the MD5 hash of "pqrstuv1048970" looks like "000006136ef...."
        assert_eq!(part1("pqrstuv")?, 1048970);

        Ok(())
    }
}
