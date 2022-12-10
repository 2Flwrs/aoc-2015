use std::collections::HashSet;

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    #[display("^")]
    Up,
    #[display("v")]
    Down,
    #[display("<")]
    Left,
    #[display(">")]
    Right,
}

impl Direction {
    fn as_vector(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[aoc_generator(day3)]
fn parse_data(input: &str) -> Result<Vec<Direction>> {
    Ok(input
        .chars()
        .map(|c| c.to_string().parse())
        .collect::<Result<Vec<Direction>, _>>()?)
}

#[aoc(day3, part1)]
fn part1(data: &[Direction]) -> Result<usize> {
    let mut cover = HashSet::new();
    let (mut x, mut y): (i32, i32) = (0, 0);
    cover.insert((x, y));

    for dir in data {
        let (dx, dy) = dir.as_vector();
        x += dx;
        y += dy;
        cover.insert((x, y));
    }

    Ok(cover.len())
}

#[aoc(day3, part2)]
fn part2(data: &[Direction]) -> Result<usize> {
    let mut cover = HashSet::new();
    let (mut x1, mut y1): (i32, i32) = (0, 0);
    let (mut x2, mut y2): (i32, i32) = (0, 0);
    cover.insert((x1, y1));
    let mut iter = data.iter();
    loop {
        if let Some(dir) = iter.next() {
            let (dx, dy) = dir.as_vector();
            x1 += dx;
            y1 += dy;
            cover.insert((x1, y1));
        } else {
            break;
        }
        if let Some(dir) = iter.next() {
            let (dx, dy) = dir.as_vector();
            x2 += dx;
            y2 += dy;
            cover.insert((x2, y2));
        } else {
            break;
        }
    }

    Ok(cover.len())
}

#[cfg(test)]
mod test {
    use super::{parse_data, part1, part2};
    use anyhow::Result;

    #[test]
    fn part1_example() -> Result<()> {
        assert_eq!(part1(&parse_data("")?)?, 1);

        // ">" delivers presents to 2 houses: one at the starting
        // location, and one to the east.
        assert_eq!(part1(&parse_data(">")?)?, 2);

        // "^>v<" delivers presents to 4 houses in a square,
        // including twice to the house at his starting/ending location.
        assert_eq!(part1(&parse_data("^>v<")?)?, 4);

        // "^v^v^v^v^v" delivers a bunch of presents to some very
        // lucky children at only 2 houses.
        assert_eq!(part1(&parse_data("^v^v^v^v^v")?)?, 2);

        Ok(())
    }

    #[test]
    fn part2_example() -> Result<()> {
        assert_eq!(part2(&parse_data("")?)?, 1);

        // "^v" delivers presents to 3 houses, because
        // Santa goes north, and then Robo-Santa goes south.
        assert_eq!(part2(&parse_data("^v")?)?, 3);

        // "^>v<" now delivers presents to 3 houses,
        // and Santa and Robo-Santa end up back where they started.
        assert_eq!(part2(&parse_data("^>v<")?)?, 3);

        // "^v^v^v^v^v" now delivers presents to 11 houses, with
        // Santa going one direction and Robo-Santa going the other.
        assert_eq!(part2(&parse_data("^v^v^v^v^v")?)?, 11);

        Ok(())
    }
}
