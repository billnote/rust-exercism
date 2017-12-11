use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

/// Expression Node
enum ENode<T> {
    Leaf(T),
    Branch {
        op: fn(a: T, b: T) -> T,
        left: Box<ENode<T>>,
        right: Box<ENode<T>>,
    },
}

/// Expression Tree
pub struct ETree<T> {
    root: ENode<T>,
}

impl<T> ENode<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Clone,
{
    pub fn evaluate(&self) -> T {
        match *self {
            ENode::Leaf(ref v) => v.clone(),
            ENode::Branch {
                op,
                ref left,
                ref right,
            } => op(left.evaluate(), right.evaluate()),
        }
    }
}

impl<T> ETree<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Clone,
{
    pub fn create(values: Vec<T>, operator: Vec<String>) -> Self {
        ETree { root: Self::create_tree(0, &values, &operator) }
    }

    pub fn evaluate(&self) -> T {
        self.root.evaluate()
    }

    fn create_tree(start: usize, values: &Vec<T>, operator: &Vec<String>) -> ENode<T> {
        let right = ENode::Leaf(values[start].clone());
        let op = Self::operate(&operator[start]);

        if start < operator.len() - 1 {
            ENode::Branch {
                op: op,
                left: Box::new(Self::create_tree(start + 1, values, operator)),
                right: Box::new(right),
            }

        } else {
            ENode::Branch {
                op: op,
                left: Box::new(ENode::Leaf(values[start + 1].clone())),
                right: Box::new(right),
            }
        }
    }

    fn operate(c: &str) -> fn(T, T) -> T {
        match c {
            "+" => Self::add,
            "-" => Self::sub,
            "*" => Self::mul,
            "/" => Self::div,
            _ => Self::unknown_operation,

        }
    }

    fn add(a: T, b: T) -> T {
        a + b
    }

    fn sub(a: T, b: T) -> T {
        a - b
    }

    fn mul(a: T, b: T) -> T {
        a * b
    }

    fn div(a: T, b: T) -> T {
        a / b
    }

    fn unknown_operation(a: T, _b: T) -> T {
        a
    }
}


#[allow(dead_code)]
pub struct WordProblem {
    question: String,
    expression: Option<ETree<i32>>,
}

impl WordProblem {
    pub fn new(command: &str) -> Self {
        if command.starts_with("What is ") && command.ends_with("?") {
            // expect exp. eg: 3 + 2 * 3
            // TODO support more operate
            let exp = command
                .to_string()
                .get_mut(8..command.len() - 1)
                .unwrap()
                .to_string()
                .replace("plus", "+")
                .replace("minus", "-")
                .replace("multiplied by", "*")
                .replace("divided by", "/");

            if exp.chars().all(|c| match c {
                '0'...'9' | '+' | '-' | '*' | '/' | ' ' => true,
                _ => false,
            })
            {

                let (items, ops): (Vec<(usize, &str)>, Vec<(usize, &str)>) =
                    exp.split_whitespace().enumerate().partition(
                        |&(idx, _)| idx % 2 == 0,
                    );

                let values: Vec<i32> = items
                    .iter()
                    .map(|&(_, v)| v.parse::<i32>().unwrap())
                    .rev()
                    .collect::<Vec<i32>>();

                let operators: Vec<String> = ops.iter()
                    .map(|&(_, v)| v.to_string())
                    .rev()
                    .collect::<Vec<String>>();

                let tree = ETree::create(values, operators);

                WordProblem {
                    question: command.to_string(),
                    expression: Some(tree),
                }

            } else {
                WordProblem {
                    question: command.to_string(),
                    expression: None,
                }
            }
        } else {
            WordProblem {
                question: command.to_string(),
                expression: None,
            }
        }
    }

    pub fn answer(&self) -> Result<i32, &'static str> {
        match self.expression {
            Some(ref exp) => Ok(exp.evaluate()),
            None => Err(("error expression.")),
        }
    }
}
