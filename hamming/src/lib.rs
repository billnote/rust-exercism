pub fn hamming_distance(dna1: &str, dna2: &str) -> Result<usize, String> {
    if dna1.len() != dna2.len() {
        Err(String::from("DNA has different length!"))
    } else {
        let dna1_bytes = dna1.as_bytes();
        let dna2_bytes = dna2.as_bytes();

        Ok(
            (0..dna1.len())
                .filter(|&idx| dna1_bytes[idx] != dna2_bytes[idx])
                .count(),
        )
    }
}
