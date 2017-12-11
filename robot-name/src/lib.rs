extern crate rand;

use rand::{thread_rng, ThreadRng, Rng};

pub struct Robot(String);

impl Robot {
    pub fn new() -> Self {
        Robot(Robot::rand_name())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        self.0 = Robot::rand_name();
    }

    fn rand_name() -> String {
        let mut rng: ThreadRng = thread_rng();
        format!(
            "{}{}{:03}",
            rng.gen_range('A' as u8, ('Z' as u8) + 1) as char,
            rng.gen_range('A' as u8, ('Z' as u8) + 1) as char,
            rng.gen_range::<u32>(0, 999)
        )

    }
}
