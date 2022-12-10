use anyhow::{anyhow, Result};
use aoc_runner_derive::aoc;
use itertools::process_results;

#[aoc(day1, part1)]
fn part1(data: &str) -> Result<isize> {
    process_results(
        data.chars().map(|c| match c {
            '(' => Ok(1),
            ')' => Ok(-1),
            c => Err(anyhow!("unexpected cahar {c:#?}")),
        }),
        |iter| iter.sum(),
    )
}

#[aoc(day1, part2)]
fn part2(data: &str) -> Result<usize> {
    let mut position = 0;
    let mut level = 0;
    for (index, step) in data
        .chars()
        .map(|c| match c {
            '(' => Ok(1),
            ')' => Ok(-1),
            c => Err(anyhow!("unexpected cahar {c:#?}")),
        })
        .enumerate()
    {
        level += step?;
        position = index;
        if level < 0 {
            break;
        }
    }
    Ok(position + 1)
}

#[cfg(test)]
mod test {
    use super::{part1, part2};
    use anyhow::Result;

    #[test]
    fn part1_s() -> Result<()> {
        let examples = [
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ];

        for (input, expected) in examples {
            assert_eq!(part1(input)?, expected);
        }

        Ok(())
    }

    #[test]
    fn part2_example() -> Result<()> {
        // ")" causes him to enter the basement at character position 1.
        assert_eq!(part2(")")?, 1);
        // "()())" causes him to enter the basement at character position 5.
        assert_eq!(part2("()())")?, 5);

        Ok(())
    }
}
