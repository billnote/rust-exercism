pub fn build_proverb(list: Vec<&str>) -> String {
    if list.is_empty() {
        String::new()
    } else {
        (0..list.len())
            .map(|index| build_proverb_rhyme(index, &list))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn build_proverb_rhyme(index: usize, list: &Vec<&str>) -> String {
    let v_len = list.len();

    if index == v_len - 1 {
        if v_len <= 2 {
            format!("And all for the want of a {}.", list[0])
        } else {
            format!("And all for the want of a horseshoe {}.", list[0])
        }
    } else {
        format!(
            "For want of a {} the {} was lost.",
            list[index],
            list[index + 1]
        )
    }
}
