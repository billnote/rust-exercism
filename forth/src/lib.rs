use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

macro_rules! binary_op {
    ($v:ident, $p1:ident, $p2:ident) => {
        let len = $v.stack.len();
        if len < 2 {
            return Err(Error::StackUnderflow);
        }
        let $p2 = $v.stack.pop().unwrap();
        let $p1 = $v.stack.pop().unwrap();
    };
    ($v:ident, $p1:ident, $p2:ident, [$($do:expr),*]) => {
        {
            binary_op!($v, $p1, $p2);
            $($do;)+
        }
    };
    ($v:ident, $p1:ident, $p2:ident, [$($do:expr),*], $($check:block)*) => {
        {
            binary_op!($v, $p1, $p2);
            $($check)*
            $($do;)+
        }
    }
}

macro_rules! unary_op {
    ($v:ident, $l:expr, $p:ident, $do:expr) => {
        {
            let len = $v.stack.len();
            if len < $l {
                return Err(Error::StackUnderflow);
            } else {
                let $p = $v.stack[len - $l];
                $do;
            }
        }
    }
}


impl Forth {
    const DEFINE_START: &'static str = ":";
    const DEFINE_END: &'static str = ";";

    pub fn new() -> Forth {
        Forth {
            stack: Vec::<Value>::new(),
            words: ["+", "-", "*", "/", "dup", "drop", "swap", "over"]
                .iter()
                .map(|op| (String::from(*op), vec![String::from(*op)]))
                .collect::<HashMap<String, Vec<String>>>(),
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    pub fn eval(&mut self, input: &'static str) -> ForthResult {
        let characters = input
            .to_lowercase()
            .split(|c: char| c.is_whitespace() || c.is_control())
            .map(|s| String::from(s))
            .collect::<Vec<String>>();

        let mut idx = 0;
        while idx < characters.len() {
            match characters[idx].as_str() {
                def_start if def_start == Forth::DEFINE_START => {
                    idx += 1;
                    if idx == characters.len() ||
                        i32::from_str_radix(&characters[idx], 10).is_ok()
                    {
                        return Err(Error::InvalidWord);
                    }

                    let new_word = characters[idx].clone();
                    let mut new_word_ops = Vec::<String>::new();

                    idx += 1;
                    if idx < characters.len() {
                        while characters[idx] != Forth::DEFINE_END {
                            new_word_ops.push(String::from(characters[idx].clone()));
                            idx += 1;

                            if idx == characters.len() {
                                return Err(Error::InvalidWord);
                            }
                        }
                    } else {
                        return Err(Error::InvalidWord);
                    }

                    self.words.insert(new_word, new_word_ops);
                }
                word if self.words.contains_key(word) => {
                    match self.op(word) {
                        Ok(_) => (),
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                numbers @ _ => {
                    match i32::from_str_radix(&numbers, 10) {
                        Ok(i) => self.stack.push(i),
                        Err(_) => {
                            return Err(Error::UnknownWord);
                        }
                    }
                }
            }
            idx += 1;
        }

        Ok(())
    }

    fn op(&mut self, word: &str) -> ForthResult {
        if let Some(ops) = self.words.get(word) {
            for op in ops {
                match op.as_str() {
                    "+" => {
                        binary_op!(self, a, b, [self.stack.push(a + b)]);
                    }
                    "-" => {
                        binary_op!(self, a, b, [self.stack.push(a - b)]);
                    }
                    "*" => {
                        binary_op!(self, a, b, [self.stack.push(a * b)]);
                    }
                    "/" => {
                        binary_op!(self, a, b, [self.stack.push(a / b)], {
                            if b == 0 {
                                return Err(Error::DivisionByZero);
                            }
                        });
                    }
                    "dup" => {
                        unary_op!(self, 1, a, self.stack.push(a));
                    }
                    "drop" => {
                        unary_op!(self, 1, _a, self.stack.pop());
                    }
                    "swap" => {
                        binary_op!(self, a, b, [self.stack.push(b), self.stack.push(a)]);
                    }
                    "over" => {
                        unary_op!(self, 2, a, self.stack.push(a));
                    }
                    number @ _ => {
                        match i32::from_str_radix(number, 10) {
                            Ok(i) => self.stack.push(i),
                            Err(_) => {
                                return Err(Error::UnknownWord);
                            }
                        }
                    }
                }
            }
        } else {
            return Err(Error::UnknownWord);
        }

        Ok(())
    }
}
