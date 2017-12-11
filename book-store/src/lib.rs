use std::collections::HashMap;

pub fn lowest_price(basket: &[usize]) -> f64 {
    if basket.len() == 0 {
        return 0.0;
    }

    let mut books = HashMap::<usize, u8>::new();
    for i in basket {
        let entry = books.entry(*i).or_insert(0);
        *entry += 1;
    }

    let mut books_vec = books
        .iter()
        .map(|(k, v)| (*k, *v))
        .collect::<Vec<(usize, u8)>>();
    books_vec.sort_by(|&(_, c1), &(_, c2)| c2.cmp(&c1));

    let books_count = books.values().sum::<u8>() as usize;
    let max_value: usize = books.len();
    let group_len = books_vec[0].1 as usize;
    let mut result = Vec::<usize>::new();
    let mut result_group = Vec::<Vec<usize>>::new();

    calc_group(
        books_count,
        max_value,
        group_len,
        &books_vec,
        &mut result,
        &mut result_group,
    );

    result_group
        .iter()
        .map(|vec| {
            vec.iter().fold(0.0f64, |acc, v| {
                let total = match *v {
                    1 => acc + 8.0,
                    2 => acc + 2.0 * 8.0 * 0.95,
                    3 => acc + 3.0 * 8.0 * 0.9,
                    4 => acc + 4.0 * 8.0 * 0.8,
                    5 => acc + 5.0 * 8.0 * 0.75,
                    _ => acc,
                };
                total
            })
        })
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

fn calc_group(
    count: usize,
    value: usize,
    len: usize,
    books: &Vec<(usize, u8)>,
    result: &mut Vec<usize>,
    result_group: &mut Vec<Vec<usize>>,
) {
    let surplus_books = calc_surplus_books(books, value);
    if value >= surplus_books.len() || (value > 0 && (surplus_books.len() / value) + 1 < len) {
        let mut new_result = result.clone();
        calc_group(count, value - 1, len, &books, &mut new_result, result_group);
    } else {
        return;
    }

    result.push(value);
    if len == result.len() {
        if count == result.iter().sum::<usize>() {
            result_group.push(result.clone());
        }
        return;
    } else {
        calc_group(
            count,
            surplus_books.len(),
            len,
            &surplus_books,
            result,
            result_group,
        );
    }
}

fn calc_surplus_books(books: &Vec<(usize, u8)>, value: usize) -> Vec<(usize, u8)> {
    books
        .iter()
        .take(value)
        .map(|&(b, v)| (b, v - 1))
        .filter(|&(_, v)| v > 0)
        .chain(books.iter().skip(value).map(|&(b, v)| (b, v)))
        .collect::<Vec<(usize, u8)>>()
}
