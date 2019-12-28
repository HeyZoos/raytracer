use crate::vec3::Vec3;
use rand::random;
use std::time::{SystemTime, UNIX_EPOCH};
use std::u64;

/// A random number generator. I chose this implementation algorithm primarily
/// because it visibly was the shortest one. Secondarily because it's supposed
/// to be "fast" and "cheap". Beyond all that I'm completely ignorant. Smite me,
/// oh mighty smiter!
///
/// https://en.wikipedia.org/wiki/Xorshift#xorshift+
pub struct Random {
    a: u64,
    b: u64,
}

impl Random {
    /// Creates a new random number generator, seeded with the current system
    /// time.
    pub fn new() -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        Random {
            a: now.as_secs(),
            b: now.as_secs(),
        }
    }

    /// Returns the next real number between 0.0 and 1.0.
    pub fn next(&mut self) -> f64 {
        let mut t = self.a;
        let s = self.b;

        self.a = s;

        t ^= t << 23;
        t ^= t >> 17;
        t ^= s ^ (s >> 26);

        self.b = t;

        (t.wrapping_add(s)) as f64 / u64::MAX as f64 // todo(heyzoos): Get rid of this division
    }

    /// Returns the next real number between bounded between `min` and `max`.
    pub fn _range(&mut self, min: f64, max: f64) -> f64 {
        self.next() * (max - min) + min
    }

    pub fn in_unit_sphere(&mut self) -> Vec3 {
        loop {
            let point = 2.0 * Vec3(random(), random(), random()) - Vec3::one();

            if point.sqlen() < 1.0 {
                return point;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::random::Random;

    #[test]
    fn next() {
        let mut random = Random::new();
        let mut sample = random.next();
        let mut last_sample = sample;

        for _ in 0..1000 {
            sample = random.next();
            assert_ne!(last_sample, sample);
            assert!(sample < 1.0);
            assert!(sample > 0.0);
            last_sample = sample;
        }
    }

    #[test]
    fn range() {
        let mut random = Random::new();
        let mut sample = random._range(-3.0, 0.0);
        let mut last_sample = sample;

        for _ in 0..1000 {
            sample = random._range(-3.0, 0.0);
            assert_ne!(last_sample, sample);
            assert!(sample < 0.0);
            assert!(sample > -3.0);
            last_sample = sample;
        }
    }
}
