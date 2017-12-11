use std::collections::BTreeMap;

pub fn transform(source: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    source
        .into_iter()
        .map(|(v, ref chars)| {
            chars
                .iter()
                .scan(*v, |state, c| {
                    Some((c.to_lowercase().next().unwrap(), *state))
                })
                .collect::<Vec<(char, i32)>>()
        })
        .collect::<Vec<Vec<(char, i32)>>>()
        .iter()
        .fold(vec![], |acc, v| {
            let mut total = acc.clone();
            let mut pairs = v.clone();
            total.append(&mut pairs);
            total
        })
        .iter()
        .map(|&(c, v)| (c, v))
        .collect()
}
