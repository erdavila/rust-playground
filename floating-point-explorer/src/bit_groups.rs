use std::collections::VecDeque;

pub type Colorizer = fn(String) -> String;

pub struct BitGroups<I>
where
    I: Iterator<Item = (String, Colorizer)>,
{
    inner: I,
    current: Option<(VecDeque<char>, Colorizer)>,
}

impl<I> BitGroups<I>
where
    I: Iterator<Item = (String, Colorizer)>,
{
    pub fn new(inner: I) -> Self {
        BitGroups {
            inner,
            current: None,
        }
    }

    fn next_from_inner(&mut self) -> Option<(VecDeque<char>, Colorizer)> {
        self.inner.next().map(|(s, colorizer)| {
            let chars = VecDeque::from_iter(s.chars());
            (chars, colorizer)
        })
    }
}

impl<I> Iterator for BitGroups<I>
where
    I: Iterator<Item = (String, Colorizer)>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut output = String::with_capacity(32);
        let mut still_missing = 8;
        while still_missing > 0 {
            match self.current.take().or_else(|| self.next_from_inner()) {
                Some((mut chars, colorizer)) => {
                    assert!(!chars.is_empty());
                    let moved_chars_len = still_missing.min(chars.len());
                    let moved_chars: String = chars.drain(..moved_chars_len).collect();
                    output.push_str(&colorizer(moved_chars));
                    still_missing -= moved_chars_len;

                    if !chars.is_empty() {
                        self.current = Some((chars, colorizer));
                    }
                }
                None if output.is_empty() => return None,
                None => break,
            }
        }

        Some(output)
    }
}

#[cfg(test)]
mod tests {
    use super::{BitGroups, Colorizer};

    fn parens(s: String) -> String {
        format!("({s})")
    }
    fn square(s: String) -> String {
        format!("[{s}]")
    }
    fn angle(s: String) -> String {
        format!("<{s}>")
    }

    #[test]
    fn case1() {
        let mut groups = BitGroups::new(
            [
                ("abcdefgh".to_string(), parens as Colorizer),
                ("ijklmnop".to_string(), square),
            ]
            .into_iter(),
        );

        assert_eq!(groups.next(), Some("(abcdefgh)".to_string()));
        assert_eq!(groups.next(), Some("[ijklmnop]".to_string()));
        assert_eq!(groups.next(), None);
    }

    #[test]
    fn case2() {
        let mut groups = BitGroups::new(
            [
                ("abcde".to_string(), parens as Colorizer),
                ("fghijklmnop".to_string(), square),
            ]
            .into_iter(),
        );

        assert_eq!(groups.next(), Some("(abcde)[fgh]".to_string()));
        assert_eq!(groups.next(), Some("[ijklmnop]".to_string()));
        assert_eq!(groups.next(), None);
    }

    #[test]
    fn case3() {
        let mut groups = BitGroups::new(
            [
                ("abc".to_string(), parens as Colorizer),
                ("de".to_string(), square),
                ("fghijklmnop".to_string(), angle),
            ]
            .into_iter(),
        );

        assert_eq!(groups.next(), Some("(abc)[de]<fgh>".to_string()));
        assert_eq!(groups.next(), Some("<ijklmnop>".to_string()));
        assert_eq!(groups.next(), None);
    }

    #[test]
    fn case4() {
        let mut groups = BitGroups::new(
            [
                ("abcdefghijk".to_string(), parens as Colorizer),
                ("lmnop".to_string(), square),
            ]
            .into_iter(),
        );

        assert_eq!(groups.next(), Some("(abcdefgh)".to_string()));
        assert_eq!(groups.next(), Some("(ijk)[lmnop]".to_string()));
        assert_eq!(groups.next(), None);
    }
}
