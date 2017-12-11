pub fn number(phone: &str) -> Option<String> {
    let numbers = phone
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();

    match numbers.len() {
        11 => {
            if numbers[0] == 1 && (numbers[1] > 1 && numbers[1] <= 9) &&
                (numbers[4] > 1 && numbers[4] <= 9)
            {
                Some(
                    numbers
                        .iter()
                        .skip(1)
                        .map(|n| n.to_string())
                        .collect::<String>(),
                )
            } else {
                None
            }
        }
        10 => {
            if (numbers[0] > 1 && numbers[0] <= 9) && (numbers[3] > 1 && numbers[3] <= 9) {
                Some(numbers.iter().map(|n| n.to_string()).collect::<String>())
            } else {
                None
            }
        }
        _ => None,
    }
}
