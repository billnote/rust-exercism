pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    const ONE_YEAR_SECONDS: f64 = 31_557_600.0;
    fn magnification() -> f64;
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (Self::ONE_YEAR_SECONDS * Self::magnification())
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn magnification() -> f64 {
        0.2408467
    }
}
impl Planet for Venus {
    fn magnification() -> f64 {
        0.61519726
    }
}
impl Planet for Earth {
    fn magnification() -> f64 {
        1.0
    }
}

impl Planet for Mars {
    fn magnification() -> f64 {
        1.8808158
    }
}
impl Planet for Jupiter {
    fn magnification() -> f64 {
        11.862615
    }
}

impl Planet for Saturn {
    fn magnification() -> f64 {
        29.447498
    }
}
impl Planet for Uranus {
    fn magnification() -> f64 {
        84.016846
    }
}
impl Planet for Neptune {
    fn magnification() -> f64 {
        164.79132
    }
}
