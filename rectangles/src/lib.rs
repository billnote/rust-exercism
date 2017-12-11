pub fn count(lines: &Vec<&str>) -> usize {
    // convert ascii pic to cartesian coordinate
    let (cc, mut points) = to_cartesion_coordinate_points(lines);
    points.reverse();
    let mut c = 0;
    while let Some(point) = points.pop() {
        c += count_iter(point, &points, &cc);
    }

    c
}

fn count_iter(
    start: (usize, usize),
    points: &Vec<(usize, usize)>,
    coordinate: &Vec<Vec<char>>,
) -> usize {
    let x_line = points
        .iter()
        .filter(|p| p.1 == start.1 && p.0 > start.0)
        .collect::<Vec<&(usize, usize)>>();
    let y_line = points
        .iter()
        .filter(|p| p.0 == start.0 && p.1 > start.1)
        .collect::<Vec<&(usize, usize)>>();

    if x_line.len() > 0 && y_line.len() > 0 {
        x_line.iter().fold(0, |acc, px| {
            let mut count = acc;
            y_line.iter().for_each(|py| if check_rectangle(
                start,
                (px.0, py.1),
                coordinate,
            )
            {
                count += 1;
            });
            count
        })
    } else {
        0
    }
}

fn check_rectangle(
    point1: (usize, usize),
    point3: (usize, usize),
    coordinate: &Vec<Vec<char>>,
) -> bool {
    coordinate[point3.1][point3.0] == '+' &&
        (point1.1..point3.1).all(|y| {
            (coordinate[y][point1.0] == '|' || coordinate[y][point1.0] == '+') &&
                (coordinate[y][point3.0] == '|' || coordinate[y][point3.0] == '+')
        }) &&
        (point1.0..point3.0).all(|x| {
            (coordinate[point1.1][x] == '-' || coordinate[point1.1][x] == '+') &&
                (coordinate[point3.1][x] == '-' || coordinate[point3.1][x] == '+')
        })
}

fn to_cartesion_coordinate_points(lines: &Vec<&str>) -> (Vec<Vec<char>>, Vec<(usize, usize)>) {
    (
        lines
            .iter()
            .rev()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),

        lines.iter().rev().enumerate().fold(
            Vec::<(usize, usize)>::new(),
            |acc, (y, s)| {
                acc.iter()
                    .map(|p| *p)
                    .chain(s.chars().enumerate().filter_map(|(x, c)| if c == '+' {
                        Some((x, y))
                    } else {
                        None
                    }))
                    .collect()
            },
        ),
    )
}
