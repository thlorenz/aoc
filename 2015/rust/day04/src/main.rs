const KEY: &str = "iwrupvqb";

fn find_hash(key: &str, n_zeroes: usize) -> (String, usize) {
    let needle = "0".repeat(n_zeroes);
    (0..)
        .into_iter()
        .map(|n| {
            let input = format!("{}{}", key, n);

            let digest = md5::compute(input.as_bytes());
            let hash_str = format!("{:?}", digest);

            if hash_str.starts_with(&needle) {
                Some((format!("{:?}", digest), n))
            } else {
                None
            }
        })
        .skip_while(Option::is_none)
        .flatten()
        .next()
        .unwrap()
}

fn main() {
    println!("5 zeroes: {:?}", find_hash(KEY, 5));
    println!("6 zeroes: {:?}", find_hash(KEY, 6));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(find_hash("abcdef", 5).1, 609043);
        assert_eq!(find_hash("pqrstuv", 5).1, 1048970);
    }
}
