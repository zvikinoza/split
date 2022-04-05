#![allow(dead_code)]

pub struct Splitter<'a, T> {
    remainder: Option<&'a str>,
    delimiter: T,
}

impl<'a, T> Splitter<'a, T> {
    pub fn new(text: &'a str, delimiter: T) -> Self {
        Self {
            remainder: Some(text),
            delimiter
        }
    }
}
pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'a, T> Iterator for Splitter<'a, T>
where
    T: Delimiter
{
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some((start, end)) = self.delimiter.find_next(remainder) {
                let until_delimiter = &remainder[..start];
                *remainder = &remainder[end..];
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
       s.char_indices()
           .find(|(_, c)| c == self)
           .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

fn until_char(s: &str, c: char) -> &str {
    Splitter::new(s, &*format!("{}", c)).next().expect("This should always work")
}

#[cfg(test)]
mod tests {
    use crate::Splitter;

    #[test]
    fn it_works() {
        let text = "a b c d e";
        let letters: Vec<_>= Splitter::new(text, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn test_trailing_delimiter() {
        let text = "a b c d ";
        let letters: Vec<_>= Splitter::new(text, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
    }

    #[test]
    fn until_char_test() {
        use super::until_char;
        assert_eq!(until_char("hello lifetimes", 'l'), "he");
    }
}
