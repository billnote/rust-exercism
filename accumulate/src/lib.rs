pub fn map_function<F>(input: Vec<i32>, call: &F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = vec![];
    for v in input {
        result.push(call(v));
    }

    result
}

pub fn map_closure<F>(input: Vec<i32>, call: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = vec![];
    for v in input {
        result.push(call(v));
    }

    result
}
