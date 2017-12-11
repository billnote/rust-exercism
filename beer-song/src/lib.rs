pub fn verse(n: i32) -> String {
    match n {
        0 => String::from(
            "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.\n",
        ),
        1 => String::from(format!(
            "{n} bottle of beer on the wall, {n} bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.\n",
            n = n
        )),
        2 => String::from(format!(
            "{n} bottles of beer on the wall, {n} bottles of beer.
Take one down and pass it around, {p} bottle of beer on the wall.\n",
            n = n,
            p = n - 1
        )),
        _ => String::from(format!(
            "{n} bottles of beer on the wall, {n} bottles of beer.
Take one down and pass it around, {p} bottles of beer on the wall.\n",
            n = n,
            p = n - 1
        )),
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut beer = String::new();
    let mut i = start;
    while i > end {
        beer.push_str(&verse(i));
        beer.push_str("\n");
        i -= 1;
    }
    beer.push_str(&verse(end));

    beer
}
