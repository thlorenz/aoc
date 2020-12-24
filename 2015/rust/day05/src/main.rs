use std::{collections::HashSet, str::Chars};

fn main() {
    let input = include_str!("input.txt");
    let nice_words = input.lines().into_iter().filter(|x| nice(x)).count();
    let nice_words_part2 = input.lines().into_iter().filter(|x| nice2(x)).count();
    println!(
        "Nice words part 1: {}, part 2: {}",
        nice_words, nice_words_part2
    );
}

fn nice(s: &str) -> bool {
    let chars: Chars = s.chars();
    let zipped = chars.clone().zip(chars.clone().into_iter().skip(1));

    let mut has_double_letter = false;
    let mut has_bad_string = false;

    for (c1, c2) in zipped.into_iter() {
        has_bad_string = match (c1, c2) {
            ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => true,
            _ => false,
        };
        if has_bad_string {
            break;
        }

        has_double_letter = has_double_letter || c1 == c2;
    }

    let vowels = chars
        .clone()
        .filter_map(|c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' => Some(()),
            _ => None,
        })
        .take(3)
        .count();
    !has_bad_string && has_double_letter && vowels == 3
}

fn nice2(s: &str) -> bool {
    let chars: Vec<_> = s.chars().collect();
    let mut repeated_with_one_between = false;
    let mut pair_found_twice = false;
    let mut pairs = HashSet::<(char, char)>::new();

    let bound = chars.len() - 2;
    for i in 0..bound {
        let (c1, c2, c3) = (chars[i], chars[i + 1], chars[i + 2]);

        if c1 == c3 {
            repeated_with_one_between = true;
        }

        if repeated_with_one_between && pair_found_twice {
            break;
        }

        if pair_found_twice {
            continue;
        }

        // Don't count overlapping pairs, i.e. only count the last 'aa' in 'aaa' or 'aaaa'
        // at which point the last 'aa', currently (c2, c3), will be (c1, c2).
        let all_equal = c1 == c2 && c2 == c3;
        if !all_equal {
            let pair = (c1, c2);
            pair_found_twice = pairs.get(&pair).is_some();
            pairs.insert(pair);
        }

        if repeated_with_one_between && pair_found_twice {
            break;
        }

        // Make sure to count the last two chars as pair if we reach the end of the string.
        let is_last_iteration = i == bound - 1;
        if is_last_iteration {
            let pair = (c2, c3);
            pair_found_twice = pairs.get(&pair).is_some();
            pairs.insert(pair);
        }

        if repeated_with_one_between && pair_found_twice {
            break;
        }
    }
    repeated_with_one_between && pair_found_twice
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_nice_cases() {
        assert!(nice("ugknbfddgicrmopn"));
        assert!(nice("aaa"));
    }

    #[test]
    fn part1_naughty_cases() {
        assert!(!nice("jchzalrnumimnmhp"));
        assert!(!nice("haegwjzuvuyypxyu"));
        assert!(!nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn part2_nice_cases() {
        assert!(nice2("xyxy"));
        assert!(nice2("xxyxx"));
        assert!(nice2("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn part2_naughty_cases() {
        assert!(!nice2("aaa"));
        assert!(!nice2("uurcxstgmygtbstg"));
        assert!(!nice2("ieodomkazucvgmuy"));
    }
}
