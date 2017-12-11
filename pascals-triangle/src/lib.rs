use std::iter;

pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            rows: (0..row_count)
                .map(|r| vec![r])
                .scan(vec![1], |state, row| {
                    if row[0] > 0 {
                        *state = iter::once(0)
                            .chain(state.clone())
                            .chain(iter::once(0))
                            .collect::<Vec<u32>>()
                            .windows(2)
                            .map(|x| x.iter().fold(0, |sum, x| sum + *x))
                            .collect::<Vec<u32>>();
                    }
                    Some(state.clone())
                })
                .collect(),
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
