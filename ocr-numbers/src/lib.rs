use std::collections::BTreeMap;
use std::collections::HashMap;
/// TODO: convert to u16
/// eg: 0
/// grid:     code:      binary:         result:
/// " _ "    (0, 1, 0)
/// "| |"    (1, 0, 1)   010101111000    1400
/// "|_|"    (1, 1, 1)
/// "   "    (0, 0, 0)
pub fn convert(input: &str) -> Result<String, ()> {
    let lines = input.lines().collect::<Vec<&str>>();
    let line_len = lines[0].len();
    if lines.len() % 4 != 0 || line_len % 3 != 0 ||
        lines.iter().any(|line| line.len() != line_len)
    {
        Err(())
    } else {
        let grid_code_map = [
            ("010101111000", '0'),
            ("000001001000", '1'),
            ("010011110000", '2'),
            ("010011011000", '3'),
            ("000111001000", '4'),
            ("010110011000", '5'),
            ("010110111000", '6'),
            ("010001001000", '7'),
            ("010111111000", '8'),
            ("010111011000", '9'),
        ].iter()
            .map(|v| *v)
            .collect::<HashMap<&str, char>>();

        let mut codes = BTreeMap::<usize, BTreeMap<usize, String>>::new();
        lines.iter().enumerate().for_each(|(line_idx, line)| {
            line.chars()
                .collect::<Vec<char>>()
                .chunks(3)
                .map(|chars| {
                    chars
                        .iter()
                        .enumerate()
                        .map(|pair| match (pair.0, *pair.1) {
                            (_, ' ') => '0',
                            (0, '|') | (2, '|') => '1',
                            (1, '_') => '1',
                            _ => '?',
                        })
                        .collect::<String>()
                })
                .enumerate()
                .for_each(|(code_idx, code_item)| {
                    let group_code = codes.entry(line_idx / 4).or_insert(
                        BTreeMap::<usize, String>::new(),
                    );
                    let code = group_code.entry(code_idx).or_insert(String::new());
                    code.push_str(&code_item);
                })
        });

        Ok(
            codes
                .values()
                .map(|codes_item| {
                    codes_item
                        .values()
                        .map(|code| *grid_code_map.get(code.as_str()).unwrap_or(&'?'))
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join(","),
        )

    }
}
