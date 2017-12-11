use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn solve(puzzle: &str) -> Option<HashMap<char, u8>> {
    let lr: Vec<&str> = puzzle.split(" == ").collect();
    if lr.len() == 2 {
        let mut left: BTreeMap<char, u32> = BTreeMap::new();
        let mut right: BTreeMap<char, u32> = BTreeMap::new();
        let mut chars: HashSet<char> = HashSet::new();
        let mut non_zero_chars: HashSet<char> = HashSet::new();

        create_exp_map(lr[0], &mut left, &mut chars, &mut non_zero_chars);
        create_exp_map(lr[1], &mut right, &mut chars, &mut non_zero_chars);

        if chars.len() > 10 {
            None
        } else {
            let mut values_map: HashMap<char, u8> = HashMap::new();
            create_value_map(
                &mut chars.iter().map(|c| *c).collect::<VecDeque<char>>(),
                &non_zero_chars,
                &mut (0..10).collect::<HashSet<u8>>(),
                &mut values_map,
                &left,
                &right,
            )
        }
    } else {
        None
    }
}

fn create_value_map(
    chars: &mut VecDeque<char>,
    non_zero_chars: &HashSet<char>,
    values: &mut HashSet<u8>,
    values_map: &mut HashMap<char, u8>,
    left: &BTreeMap<char, u32>,
    right: &BTreeMap<char, u32>,
) -> Option<HashMap<char, u8>> {

    if let Some(c) = chars.pop_back() {
        for i in values.iter() {
            if *i == 0 && non_zero_chars.contains(&c) {
                continue;
            }
            values_map.insert(c, *i);
            let mut new_values = values.clone();
            new_values.remove(&i);

            if let Some(result) = create_value_map(
                &mut chars.clone(),
                non_zero_chars,
                &mut new_values,
                &mut values_map.clone(),
                &left,
                &right,
            )
            {
                return Some(result);
            }
        }
    } else {
        if calc_exp_value(&left, values_map) == calc_exp_value(&right, values_map) {
            return Some(
                values_map
                    .iter()
                    .map(|(k, v)| (*k, *v))
                    .collect::<HashMap<char, u8>>(),
            );
        }
    }

    None
}


fn create_exp_map(
    exp: &str,
    exp_map: &mut BTreeMap<char, u32>,
    char_set: &mut HashSet<char>,
    non_zero_chars: &mut HashSet<char>,
) {
    exp.split(" + ").for_each(|s| {
        let max_idx = s.len() - 1;
        s.chars().rev().enumerate().for_each(|(idx, c)| {
            char_set.insert(c);
            if max_idx == idx {
                non_zero_chars.insert(c);
            }
            let entry = exp_map.entry(c).or_insert(0);
            *entry += 10u32.pow(idx as u32);
        })
    });
}

fn calc_exp_value(exp_map: &BTreeMap<char, u32>, value_map: &HashMap<char, u8>) -> u32 {
    let mut result = 0u32;
    exp_map.iter().fold(&mut result, |acc, (k, v)| {
        let value = value_map.get(k).unwrap();
        *acc += v * (*value as u32);
        acc
    });

    result
}
