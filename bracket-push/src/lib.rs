use std::convert::From;
use std::collections::VecDeque;

pub struct Brackets {
    statement: String,
}

impl From<&'static str> for Brackets {
    fn from(s: &str) -> Self {
        Brackets { statement: s.to_string() }
    }
}

impl Brackets {
    pub const BRACKETS: &'static [(char, char)] = &[('(', ')'), ('[', ']'), ('{', '}')];

    pub fn are_balanced(&self) -> bool {
        let mut bks: VecDeque<char> = VecDeque::new();
        let mut is_balanced = true;

        for c in self.statement.chars() {
            match c {
                b if Brackets::is_left_bracket(b) => {
                    bks.push_front(c);
                }
                b if Brackets::is_right_bracket(b) => {
                    if !Brackets::is_bracket_pair((bks.pop_front().unwrap_or(' '), c)) {
                        is_balanced = false;
                        break;
                    }
                }
                _ => {
                    continue;
                }
            }
        }

        is_balanced && bks.len() == 0
    }

    fn is_left_bracket(c: char) -> bool {
        Brackets::BRACKETS.iter().any(|&(l, _)| l == c)
    }

    fn is_right_bracket(c: char) -> bool {
        Brackets::BRACKETS.iter().any(|&(_, r)| r == c)
    }

    fn is_bracket_pair(pair: (char, char)) -> bool {
        Brackets::BRACKETS.iter().any(|b| *b == pair)
    }
}
