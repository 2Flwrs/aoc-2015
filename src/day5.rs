use std::collections::HashSet;

use anyhow::Result;
use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day5, part1)]
fn part1(data: &str) -> Result<usize> {
    Ok(data
        .lines()
        .filter_map(|word| is_nice_part1(word).then_some(()))
        .count())
}

fn is_nice_part1(word: &str) -> bool {
    has_3_vowels(word) && has_double_letter(word) && no_forbidden_pair(word)
}

fn has_3_vowels(word: &str) -> bool {
    const VOWELS: &str = "aeiou";
    word.chars()
        .filter_map(|c| VOWELS.contains(c).then_some(()))
        .count()
        >= 3
}

fn has_double_letter(word: &str) -> bool {
    word.chars().tuple_windows().any(|(a, b)| a == b)
}

fn no_forbidden_pair(word: &str) -> bool {
    const FORBIDDEN: [(char, char); 4] = [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];
    word.chars()
        .tuple_windows()
        .all(|(a, b)| !FORBIDDEN.iter().cloned().any(|(x, y)| a == x && b == y))
}

#[aoc(day5, part2)]
fn part2(data: &str) -> Result<usize> {
    Ok(data
        .lines()
        .filter_map(|word| is_nice_part2(word).then_some(()))
        .count())
}

fn is_nice_part2(word: &str) -> bool {
    has_two_pairs(word) && has_double_spaced(word)
}

fn has_double_spaced(word: &str) -> bool {
    word.chars().tuple_windows().any(|(a, _, b)| a == b)
}

fn has_two_pairs(word: &str) -> bool {
    let mut pairs = HashSet::new();
    let mut last = ' ';

    for pair @ (a, b) in word.chars().tuple_windows() {
        let tripple = (a == b) && (a == last);
        if !pairs.insert(pair) && !tripple {
            return true;
        }

        last = a;
    }

    for (a, b, c, d) in word.chars().tuple_windows() {
        if a == b && b == c && c == d {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::{has_double_spaced, has_two_pairs, is_nice_part1, is_nice_part2};
    use anyhow::Result;

    #[test]
    fn part1_example() -> Result<()> {
        // "ugknbfddgicrmopn" is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and none of the disallowed substrings.
        assert_eq!(is_nice_part1("ugknbfddgicrmopn"), true);
        // "aaa" is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.
        assert_eq!(is_nice_part1("aaa"), true);
        // "jchzalrnumimnmhp" is naughty because it has no double letter.
        assert_eq!(is_nice_part1("jchzalrnumimnmhp"), false);
        // "haegwjzuvuyypxyu" is naughty because it contains the string xy.
        assert_eq!(is_nice_part1("haegwjzuvuyypxyu"), false);
        // "dvszwmarrgswjxmb" is naughty because it contains only one vowel.
        assert_eq!(is_nice_part1("dvszwmarrgswjxmb"), false);

        Ok(())
    }

    #[test]
    fn part2_example() -> Result<()> {
        // "qjhvhtzxzqqjkmpb" is nice because is has a pair that appears twice (qj) and a letter that repeats with exactly one letter between them (zxz).
        assert_eq!(has_two_pairs("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(has_double_spaced("qjhvhtzxzqqjkmpb"), true);
        // "xxyxx" is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.
        assert_eq!(has_two_pairs("xxyxx"), true);
        assert_eq!(has_double_spaced("xxyxx"), true);
        // "uurcxstgmygtbstg" is naughty because it has a pair (tg) but no repeat with a single letter between them.
        assert_eq!(has_two_pairs("uurcxstgmygtbstg"), true);
        assert_eq!(has_double_spaced("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice_part2("uurcxstgmygtbstg"), false);
        // "ieodomkazucvgmuy" is naughty because it has a repeating letter with one between (odo), but no pair that appears twice.
        assert_eq!(has_two_pairs("ieodomkazucvgmuy"), false);
        assert_eq!(has_double_spaced("ieodomkazucvgmuy"), true);
        assert_eq!(is_nice_part2("ieodomkazucvgmuy"), false);

        Ok(())
    }
}
