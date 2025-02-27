//! Different types of delay for retryable operations.

use std::ops::{Range as StdRange, RangeInclusive};
use std::time::Duration;
use std::u64::MAX as U64_MAX;

use rand::{
    distr::{Distribution, Uniform},
    random, rng,
    rngs::ThreadRng,
};

/// Each retry increases the delay since the last exponentially.
#[derive(Debug)]
pub struct Exponential {
    base: u64,
    current: u64,
}

impl Exponential {
    /// Create a new `Exponential` using the given millisecond duration as the initial delay.
    pub fn from_millis(base: u64) -> Self {
        Exponential {
            base,
            current: base,
        }
    }
}

impl Iterator for Exponential {
    type Item = Duration;

    fn next(&mut self) -> Option<Duration> {
        let duration = Duration::from_millis(self.current);

        if let Some(next) = self.current.checked_mul(self.base) {
            self.current = next;
        } else {
            self.current = U64_MAX;
        }

        Some(duration)
    }
}

impl From<Duration> for Exponential {
    fn from(duration: Duration) -> Self {
        Self::from_millis(duration.as_millis() as u64)
    }
}

/// Each retry uses a delay which is the sum of the two previous delays.
///
/// Depending on the problem at hand, a fibonacci delay strategy might
/// perform better and lead to better throughput than the `Exponential`
/// strategy.
///
/// See ["A Performance Comparison of Different Backoff Algorithms under Different Rebroadcast Probabilities for MANETs."](http://www.comp.leeds.ac.uk/ukpew09/papers/12.pdf)
/// for more details.
#[derive(Debug)]
pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    /// Create a new `Fibonacci` using the given duration in milliseconds.
    pub fn from_millis(millis: u64) -> Fibonacci {
        Fibonacci {
            curr: millis,
            next: millis,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = Duration;

    fn next(&mut self) -> Option<Duration> {
        let duration = Duration::from_millis(self.curr);

        if let Some(next_next) = self.curr.checked_add(self.next) {
            self.curr = self.next;
            self.next = next_next;
        } else {
            self.curr = self.next;
            self.next = U64_MAX;
        }

        Some(duration)
    }
}

impl From<Duration> for Fibonacci {
    fn from(duration: Duration) -> Self {
        Self::from_millis(duration.as_millis() as u64)
    }
}

#[test]
fn fibonacci() {
    let mut iter = Fibonacci::from_millis(10);
    assert_eq!(iter.next(), Some(Duration::from_millis(10)));
    assert_eq!(iter.next(), Some(Duration::from_millis(10)));
    assert_eq!(iter.next(), Some(Duration::from_millis(20)));
    assert_eq!(iter.next(), Some(Duration::from_millis(30)));
    assert_eq!(iter.next(), Some(Duration::from_millis(50)));
    assert_eq!(iter.next(), Some(Duration::from_millis(80)));
}

#[test]
fn fibonacci_saturated() {
    let mut iter = Fibonacci::from_millis(U64_MAX);
    assert_eq!(iter.next(), Some(Duration::from_millis(U64_MAX)));
    assert_eq!(iter.next(), Some(Duration::from_millis(U64_MAX)));
}

/// Each retry uses a fixed delay.
#[derive(Debug)]
pub struct Fixed {
    duration: Duration,
}

impl Fixed {
    /// Create a new `Fixed` using the given duration in milliseconds.
    pub fn from_millis(millis: u64) -> Self {
        Fixed {
            duration: Duration::from_millis(millis),
        }
    }
}

impl Iterator for Fixed {
    type Item = Duration;

    fn next(&mut self) -> Option<Duration> {
        Some(self.duration)
    }
}

impl From<Duration> for Fixed {
    fn from(delay: Duration) -> Self {
        Self { duration: delay }
    }
}

/// Each retry happens immediately without any delay.
#[derive(Debug)]
pub struct NoDelay;

impl Iterator for NoDelay {
    type Item = Duration;

    fn next(&mut self) -> Option<Duration> {
        Some(Duration::default())
    }
}

/// Each retry uses a duration randomly chosen from a range.
#[derive(Debug)]
pub struct Range {
    distribution: Uniform<u64>,
    rng: ThreadRng,
}

impl Range {
    /// Create a new `Range` between the given millisecond durations, excluding the maximum value.
    ///
    /// # Panics
    ///
    /// Panics if the minimum is greater than or equal to the maximum.
    pub fn from_millis_exclusive(minimum: u64, maximum: u64) -> Self {
        Range {
            distribution: Uniform::new(minimum, maximum).expect("Invalid Inputs"),
            rng: rng(),
        }
    }

    /// Create a new `Range` between the given millisecond durations, including the maximum value.
    ///
    /// # Panics
    ///
    /// Panics if the minimum is greater than or equal to the maximum.
    pub fn from_millis_inclusive(minimum: u64, maximum: u64) -> Self {
        Range {
            distribution: Uniform::new_inclusive(minimum, maximum).expect("Invalid Inputs"),
            rng: rng(),
        }
    }
}

impl Iterator for Range {
    type Item = Duration;

    fn next(&mut self) -> Option<Duration> {
        Some(Duration::from_millis(
            self.distribution.sample(&mut self.rng),
        ))
    }
}

impl From<StdRange<Duration>> for Range {
    fn from(range: StdRange<Duration>) -> Self {
        Self::from_millis_exclusive(range.start.as_millis() as u64, range.end.as_millis() as u64)
    }
}

impl From<RangeInclusive<Duration>> for Range {
    fn from(range: RangeInclusive<Duration>) -> Self {
        Self::from_millis_inclusive(
            range.start().as_millis() as u64,
            range.end().as_millis() as u64,
        )
    }
}

/// Apply full random jitter to a duration.
pub fn jitter(duration: Duration) -> Duration {
    let jitter = random::<f64>();
    let secs = ((duration.as_secs() as f64) * jitter).ceil() as u64;
    let nanos = ((f64::from(duration.subsec_nanos())) * jitter).ceil() as u32;
    Duration::new(secs, nanos)
}
