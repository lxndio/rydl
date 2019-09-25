pub fn split_string_every(s: &str, n: usize) -> Vec<String> {
    let mut res = Vec::new();

    let mut ctr = 0;
    while ctr < s.len() {
        res.push(s.chars().skip(ctr).take(n).collect());

        ctr += n;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_string_every_test() {
        let s = "abc123abc123abc123abc123abc12";
        let n = 3;

        let res = split_string_every(s, n);

        let res_expected = vec! {
            String::from("abc"),
            String::from("123"),
            String::from("abc"),
            String::from("123"),
            String::from("abc"),
            String::from("123"),
            String::from("abc"),
            String::from("123"),
            String::from("abc"),
            String::from("12"),
        };

        assert_eq!(res_expected, res);
    }
}