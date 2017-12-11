#[derive(PartialEq)]
#[derive(Debug)]
pub enum Comparison {
    Sublist,
    Superlist,
    Equal,
    Unequal,
}

pub fn sublist<T>(a: &[T], b: &[T]) -> Comparison
where
    T: Eq,
{
    if a.len() >= b.len() {
        sublist_by_len(a, b)
    } else {
        reserve(sublist_by_len(b, a))
    }
}

fn reserve(c: Comparison) -> Comparison {
    match c {
        Comparison::Sublist => Comparison::Superlist,
        Comparison::Superlist => Comparison::Sublist,
        _ => c,
    }
}

fn sublist_by_len<T>(a: &[T], b: &[T]) -> Comparison
where
    T: Eq,
{
    let mut n: usize = 0;
    let mut m: usize = 0;

    while n < a.len() && m < b.len() && n >= m {
        if a[n] == b[m] {
            m += 1;
        } else {
            n -= m;
            m = 0;
        }
        n += 1;
    }

    if m == b.len() {
        if a.len() == b.len() {
            Comparison::Equal
        } else {
            Comparison::Superlist
        }
    } else {
        Comparison::Unequal
    }
}
