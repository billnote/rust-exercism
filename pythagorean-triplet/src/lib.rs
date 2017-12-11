pub fn find() -> Option<u32> {
    /*
    
    ∵
      a + b + c = 1_000
      a.pow(2) + b.pow(2) = c.pow(2)
    ∴
      a*b*c = c * (500_000 - 1_000 * c)
      a*b*c = 1000 * c (500 -c)
    ∵
      a b c is natural numbers
    ∴
      c < 500 & c >= 334
    
     */

    for c in 334u32..500u32 {
        for b in (0u32..334u32).rev() {
            let a = 1_000 - c - b;
            if a >= c {
                break;
            }
            if a.pow(2) + b.pow(2) == c.pow(2) {
                return Some(a * b * c);
            }
        }
    }

    None
}
