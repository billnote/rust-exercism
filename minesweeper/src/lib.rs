pub fn annotate(board: &Vec<&str>) -> Vec<String> {
    if board.len() == 0 {
        Vec::<String>::new()
    } else if board[0].len() == 0 {
        vec![String::from("")]
    } else {
        let max_y = board.len() - 1;
        let max_x = board[0].len() - 1;
        board
            .iter()
            .enumerate()
            .map(|(y, r)| {
                r.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '*' => String::from("*"),
                        _ => {
                            let neighborhood = calc_neighboring_point(&(x, y), &(max_x, max_y));
                            let value = neighborhood.iter().fold(0, |acc, point| {
                                let mut total = acc;
                                if board[point.1].chars().nth(point.0).unwrap_or(' ') == '*' {
                                    total += 1;
                                }
                                total
                            });

                            if value == 0 {
                                String::from(" ")
                            } else {
                                value.to_string()
                            }
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
    }
}


fn calc_neighboring_point(
    point: &(usize, usize),
    max_point: &(usize, usize),
) -> Vec<(usize, usize)> {
    NEIGHBORHOOD
        .iter()
        .filter_map(|&(x, y)| {
            let nb_point = (point.0 as i32 + x, point.1 as i32 + y);
            correct_point(&nb_point, max_point)
        })
        .collect::<Vec<(usize, usize)>>()
}

fn correct_point(point: &(i32, i32), max_point: &(usize, usize)) -> Option<(usize, usize)> {
    if point.0 >= 0 && point.1 >= 0 && point.0 <= max_point.0 as i32 &&
        point.1 <= max_point.1 as i32
    {
        Some((point.0 as usize, point.1 as usize))
    } else {
        None
    }
}

static NEIGHBORHOOD: &[(i32, i32)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];
