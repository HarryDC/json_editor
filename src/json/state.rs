use super::error::Error;
use super::error::Error::{Character, EndOfLine};

pub struct State {
    content: Vec<char>,
    cursor: usize
}

impl State {
    pub fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            content: string.chars().collect(),
        }
    }

    pub fn error(&self) -> Error {
        if self.at_end() {
            EndOfLine
        } else {
            Character(self.cursor)
        }
    }

    pub fn at_end(&self) -> bool {
        self.cursor >= self.content.len()
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn peek(&self) -> Option<&char> {
        self.content.get(self.cursor)
    }

    pub fn take(&mut self) -> Option<&char> {
        match self.content.get(self.cursor) {
            Some(character) => {
                self.cursor += 1;
                Some(character)
            }
            None => None,
        }
    }

    // Return true if we're at the end of the string
    pub fn skip_whitespace(&mut self) -> bool {
        while self.peek().is_some_and(|x| x.is_whitespace()) {
            self.take();
        }
        self.at_end()
    }

    pub fn read_char(&mut self, c : char) -> bool {
        let mut result = false;
        if !self.skip_whitespace() && self.peek() == Some(&c) {
            result = true;
            self.cursor += 1
        }
        result
    }

    pub fn read_literal(&mut self, lit : &str) -> bool {
        if self.skip_whitespace() {
            return false;
        }

        for c in lit.chars() {
            if self.peek() != Some(&c) {
                return false;
            }
            self.cursor+=1;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_state_read_char() {
        let mut state_1 = State::new(" te ");
        assert_eq!(true, state_1.read_char('t'));
        assert_eq!(false, state_1.read_char('t'));
        assert_eq!(true, state_1.read_char('e'));
        assert_eq!(false, state_1.read_char('e'));
        assert_eq!(true, state_1.at_end());
    }

    #[test]
    fn test_state_read_literal() {
        let mut state_1 = State::new("  true ");
        assert_eq!(true, state_1.read_literal("true"));


        let mut state_2 = State::new("  true ");
        assert_eq!(false, state_2.read_literal("trx"));

        let mut state_3 = State::new("false  true ");
        assert_eq!(true, state_3.read_literal("false"));
        assert_eq!(true, state_3.read_literal("true"));
    }

    #[test]
    fn test_state_skip_whitespace() {
        let mut state_1 = State::new("test");
        assert_eq!(false, state_1.skip_whitespace());
        assert_eq!(0, state_1.cursor);
        assert_eq!(false, state_1.at_end());
        assert_eq!(Some(&'t'), state_1.peek());

        let mut state_2 = State::new("   test");
        assert_eq!(false, state_2.skip_whitespace());
        assert_eq!(3, state_2.cursor);
        assert_eq!(false, state_2.at_end());
        assert_eq!(Some(&'t'), state_2.peek());

        let mut state_3 = State::new("t    ");
        assert_eq!(Some(&'t'), state_3.take());
        assert_eq!(true, state_3.skip_whitespace());
        assert_eq!(true, state_3.at_end());
    }
}