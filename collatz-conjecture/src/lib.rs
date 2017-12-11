// return Ok(x) where x is the number of steps required to reach 1
pub fn collatz(n: u64) -> Result<u64, &'static str> {
    if n == 0 {
        return Err("given number must gt zero!");
    }

    let mut result = n;
    let mut step_count = 0;

    while result != 1 {
        let q = result % 2;
        if q == 0 {
            result /= 2;
        } else {
            result = 3 * result + 1;
        }

        step_count += 1;
    }

    Ok(step_count)
}
