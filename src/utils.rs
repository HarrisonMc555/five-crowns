pub fn split_first_char(string: &str) -> Option<(char, String)> {
    let mut chars = string.chars();
    let first_char = chars.next()?;
    let rest = chars.collect();
    Some((first_char, rest))
}

pub fn split_last_char(string: &str) -> Option<(String, char)> {
    let mut rest = Vec::new();
    let mut chars = string.chars();
    let mut last_char = chars.next()?;
    for c in chars {
        rest.push(last_char);
        last_char = c;
    }
    let rest = rest.into_iter().collect();
    Some((rest, last_char))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_first_char() {
        assert_eq!(Some(('a', "sdf".to_string())), split_first_char("asdf"));
        assert_eq!(Some(('a', "".to_string())), split_first_char("a"));
        assert_eq!(Some(('H', "i Bob".to_string())), split_first_char("Hi Bob"));
        assert_eq!(None, split_first_char(""));
    }

    #[test]
    fn test_split_last_char() {
        assert_eq!(Some(("asd".to_string(), 'f')), split_last_char("asdf"));
        assert_eq!(Some(("".to_string(), 'a')), split_last_char("a"));
        assert_eq!(Some(("Hi Bo".to_string(), 'b')), split_last_char("Hi Bob"));
        assert_eq!(None, split_last_char(""));
    }
}
