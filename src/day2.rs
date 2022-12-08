use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug, Clone, Copy, PartialEq, Eq)]
#[display("{0}x{1}x{2}")]
struct Present(usize, usize, usize);

impl Present {
    fn as_slice(&self) -> [usize; 3] {
        [self.0, self.1, self.2]
    }

    fn paper_needed(&self) -> usize {
        let areas = self
            .as_slice()
            .iter()
            .tuple_combinations()
            .map(|(&a, &b)| a * b)
            .collect::<Vec<_>>();
        2 * areas.iter().sum::<usize>() + areas.iter().min().unwrap()
    }

    fn ribbon_needed(&self) -> usize {
        let min_perimeter = self
            .as_slice()
            .iter()
            .tuple_combinations()
            .map(|(&a, &b)| 2 * (a + b))
            .min()
            .unwrap();
        let volume = self.as_slice().iter().product::<usize>();
        min_perimeter + volume
    }
}

#[aoc_generator(day2)]
fn parse_data(input: &str) -> Result<Vec<Present>> {
    Ok(input
        .lines()
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()?)
}

#[aoc(day2, part1)]
fn part1(data: &[Present]) -> Result<usize> {
    Ok(data.iter().map(|p| p.paper_needed()).sum())
}

#[aoc(day2, part2)]
fn part2(data: &[Present]) -> Result<usize> {
    Ok(data.iter().map(|p| p.ribbon_needed()).sum())
}

#[cfg(test)]
mod test {
    use super::{parse_data, part1, part2};
    use anyhow::Result;

    #[test]
    fn part1_example() -> Result<()> {
        // A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet
        // of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
        assert_eq!(part1(&parse_data("2x3x4")?)?, 58);

        // A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet
        // of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.
        assert_eq!(part1(&parse_data("1x1x10")?)?, 43);

        Ok(())
    }

    #[test]
    fn part2_example() -> Result<()> {
        // A present with dimensions 2x3x4 requires 2+2+3+3 = 10 feet of ribbon to wrap the
        // present plus 2*3*4 = 24 feet of ribbon for the bow, for a total of 34 feet.
        assert_eq!(part2(&parse_data("2x3x4")?)?, 34);
        // A present with dimensions 1x1x10 requires 1+1+1+1 = 4 feet of ribbon to wrap the
        // present plus 1*1*10 = 10 feet of ribbon for the bow, for a total of 14 feet.
        assert_eq!(part2(&parse_data("1x1x10")?)?, 14);
        Ok(())
    }
}
