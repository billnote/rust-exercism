use std::fmt::Debug;

pub fn find<T>(sorted_list: &[T], x: T) -> Option<usize>
where
    T: Ord + Debug,
{
    if sorted_list.len() == 0 {
        None
    } else {
        find_iter(sorted_list, &x, sorted_list.len() / 2)
    }
}


fn find_iter<T>(sorted_list: &[T], x: &T, mid_idx: usize) -> Option<usize>
where
    T: Ord + Debug,
{
    let (p1, p2) = sorted_list.split_at(sorted_list.len() / 2);

    match x {
        v if *v == *p2.first().unwrap() => Some(mid_idx),
        v if is_in_range(p2, v) => find_iter(p2, x, mid_idx + p2.len() / 2),
        v if *v == *p2.last().unwrap() => Some(mid_idx + p2.len() - 1),
        v if *v == *p1.first().unwrap() => Some(mid_idx - p1.len()),
        v if is_in_range(p1, v) => find_iter(p1, x, mid_idx - p1.len() / 2),
        v if *v == *p1.last().unwrap() => Some(mid_idx - 1),
        _ => None,
    }
}

fn is_in_range<T>(sorted_list: &[T], x: &T) -> bool
where
    T: Ord,
{
    *x < *sorted_list.last().unwrap() && *x > *sorted_list.first().unwrap()
}
