use std::collections::HashMap;

pub fn count(nucleotide: char, dna_snippet: &str) -> Result<usize, String> {
    if !is_nucleotide(nucleotide) || dna_snippet.chars().any(|c| !is_nucleotide(c)) {
        Err(String::from("error symbol!"))
    } else {
        Ok(dna_snippet.chars().filter(|c| *c == nucleotide).count())
    }
}

pub fn nucleotide_counts(dna_snippet: &str) -> Result<HashMap<char, usize>, String> {
    if dna_snippet.chars().all(|c| is_nucleotide(c)) {
        let mut map = [('A', 0), ('C', 0), ('G', 0), ('T', 0)]
            .iter()
            .map(|&(c, v)| (c, v))
            .collect::<HashMap<char, usize>>();

        Ok(
            dna_snippet
                .chars()
                .fold(&mut map, |acc, c| {
                    let count = *acc.get(&c).unwrap_or(&0);
                    acc.insert(c, count + 1);
                    acc
                })
                .clone(),
        )
    } else {
        Err(String::from("error symbol!"))
    }
}

fn is_nucleotide(nucleotide: char) -> bool {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false,
    }
}
