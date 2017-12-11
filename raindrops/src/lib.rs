pub fn raindrops(n: usize) -> String {
    let mut result: String = String::new();
    if n % 3 == 0 {
        result += "Pling";
    }
    if n % 5 == 0 {
        result += "Plang";
    }
    if n % 7 == 0 {
        result += "Plong";
    }

    if result.len() == 0 {
        result = n.to_string();
    }

    result
}
