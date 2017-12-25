pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input.iter().enumerate().fold(Vec::new(), |acc, item| {
        acc.iter()
            .map(|p| *p)
            .chain(max_idx(item.1).iter().filter_map(|idx| {
                if input[item.0][*idx] == *column_n(*idx, input).iter().min().unwrap() {
                    Some((item.0, *idx))
                } else {
                    None
                }
            }))
            .collect::<Vec<(usize, usize)>>()
    })
}

fn _min_idx(input: &[u64]) -> Vec<usize> {
    idx_by(input, |a, b| a > b)
}

fn max_idx(input: &[u64]) -> Vec<usize> {
    idx_by(input, |a, b| a < b)
}

fn column_n(n: usize, input: &[Vec<u64>]) -> Vec<u64> {
    (0..input.len()).map(|i| input[i][n]).collect::<Vec<u64>>()
}

fn idx_by<F: Fn(u64, u64) -> bool>(input: &[u64], f: F) -> Vec<usize> {
    let mut max_idx_vec = Vec::new();
    input.iter().enumerate().fold(
        (&mut max_idx_vec, &mut 0u64),
        |acc, item| {
            if f(*acc.1, *item.1) {
                acc.0.clear();
                *acc.1 = *item.1;
                acc.0.push(item.0);
            } else if *acc.1 == *item.1 {
                acc.0.push(item.0)
            }
            acc
        },
    );

    max_idx_vec
}
