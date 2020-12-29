fn main() {
    let input = include_str!("./input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let (in_code, in_memory) = process_list_part1(&lines);
    let diff = in_code - in_memory;
    println!("part1: {}", diff);

    let (in_code, encoded) = process_list_part2(&lines);
    let diff = encoded - in_code;
    println!("part2: {}", diff);
}

fn process_list_part1(list: &Vec<&str>) -> (usize, usize) {
    list.into_iter()
        .fold((0, 0), |(total_in_code, total_in_memory), line| {
            let (in_code, in_memory) = string_lengths_part1(line);
            (total_in_code + in_code, total_in_memory + in_memory)
        })
}

fn string_lengths_part1(line: &str) -> (usize, usize) {
    let chars: Vec<char> = line.chars().collect();
    let in_code = chars.len();
    let mut in_memory = 0;
    let mut skip = 0;
    let upper = in_code - 1;

    for i in 1..upper {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        let c = chars[i];
        if let '\\' = c {
            let nc = chars[i + 1];
            match nc {
                '\\' | '"' => {
                    in_memory += 1;
                    skip = 1;
                }
                'x' => skip = 2,
                _ => panic!(
                    "Saw escape character followed by unescapable char when processing '{}'",
                    line
                ),
            }
        } else {
            in_memory += 1;
        }
    }
    (in_code, in_memory)
}

fn process_list_part2(list: &Vec<&str>) -> (usize, usize) {
    list.into_iter()
        .fold((0, 0), |(total_in_code, total_encoded), line| {
            let (in_code, encoded) = string_lengths_part2(line);
            (total_in_code + in_code, total_encoded + encoded)
        })
}

fn string_lengths_part2(line: &str) -> (usize, usize) {
    let chars: Vec<char> = line.chars().collect();
    let in_code = chars.len();
    let mut encoded = 6;
    let mut skip = 0;
    let upper = in_code - 1;

    for i in 1..upper {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        let c = chars[i];
        if let '\\' = c {
            let nc = chars[i + 1];
            match nc {
                '\\' | '"' => {
                    encoded += 4;
                    skip = 1;
                }
                'x' => {
                    encoded += 4;
                    skip = 2;
                }
                _ => panic!(
                    "Saw escape character followed by unescapable char when processing '{}'",
                    line
                ),
            }
        } else {
            encoded += 1;
        }
    }
    (in_code, encoded)
}

#[cfg(test)]
mod test_part1 {
    use super::*;

    #[test]
    fn empty_string() {
        let (in_code, in_memory) = string_lengths_part1("\"\"");
        assert_eq!((in_code, in_memory), (2, 0));
    }

    #[test]
    fn abc() {
        let (in_code, in_memory) = string_lengths_part1("\"abc\"");
        assert_eq!((in_code, in_memory), (5, 3));
    }

    #[test]
    fn with_escaped_quote() {
        let (in_code, in_memory) = string_lengths_part1("\"aaa\\\"aaa\"");
        assert_eq!((in_code, in_memory), (10, 7));
    }

    #[test]
    fn escaped_apostrophe() {
        let (in_code, in_memory) = string_lengths_part1("\"\\x27\"");
        assert_eq!((in_code, in_memory), (6, 1));
    }

    #[test]
    fn samples() {
        let input = include_str!("./sample.txt");
        let lines: Vec<&str> = input.lines().collect();
        let (in_code, in_memory) = process_list_part1(&lines);
        assert_eq!((in_code, in_memory), (23, 11));
    }
}

#[cfg(test)]
mod test_part2 {
    use super::*;

    #[test]
    fn empty_string() {
        let (in_code, in_memory) = string_lengths_part2("\"\"");
        assert_eq!((in_code, in_memory), (2, 6));
    }

    #[test]
    fn abc() {
        let (in_code, in_memory) = string_lengths_part2("\"abc\"");
        assert_eq!((in_code, in_memory), (5, 9));
    }

    #[test]
    fn with_escaped_quote() {
        let (in_code, in_memory) = string_lengths_part2("\"aaa\\\"aaa\"");
        assert_eq!((in_code, in_memory), (10, 16));
    }

    #[test]
    fn escaped_apostrophe() {
        let (in_code, in_memory) = string_lengths_part2("\"\\x27\"");
        assert_eq!((in_code, in_memory), (6, 11));
    }

    #[test]
    fn samples() {
        let input = include_str!("./sample.txt");
        let lines: Vec<&str> = input.lines().collect();
        let (in_code, in_memory) = process_list_part2(&lines);
        assert_eq!((in_code, in_memory), (23, 42));
    }
}
