#[derive(Debug, PartialEq)]
pub struct BackoffAlgorithm {
    // The maximum backoff base (in milliseconds) between consecutive retry attempts.
    pub max: usize,
    // The total number of retry attempts completed.
    // This value is incremented on every call to #BackoffAlgorithm_GetNextBackoff API.
    // pub attemptsDone: usize,
    // The maximum backoff value (in milliseconds) for the next retry attempt.
    pub base: usize,
    // The maximum number of retry attempts.
    // pub maxRetryAttempts: usize,
    power: usize,
    pub value: usize,
    pub rand: Option<usize>,
}

impl BackoffAlgorithm {
    pub fn new(base: usize, max: usize, rand: Option<usize>) -> BackoffAlgorithm {
        BackoffAlgorithm {
            base,
            max,
            power: base,
            value: base,
            rand,
        }
    }
    pub fn get(&self) -> usize {
        self.value
    }
}

impl Iterator for BackoffAlgorithm {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = self.power + self.rand.unwrap_or_default() % self.power;
        self.power += self.power;

        if self.value <= self.max {
            Some(self.value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::random;

    use crate::backoff_algo;
    #[test]
    fn next_test() {
        let mut bfa = backoff_algo::BackoffAlgorithm::new(1, 16, None);
        assert_eq!(bfa.next(), Some(1));
        assert_eq!(bfa.get(), 1);
        assert_eq!(bfa.next(), Some(2));
        assert_eq!(bfa.get(), 2);
        assert_eq!(bfa.next(), Some(4));
        assert_eq!(bfa.get(), 4);
        assert_eq!(bfa.next(), Some(8));
        assert_eq!(bfa.next(), Some(16));
        assert_eq!(bfa.get(), 16);
        assert_eq!(bfa.next(), None);
        assert_eq!(bfa.get(), 32);
    }
    #[test]
    fn next_with_random_test() {
        let mut bfa = backoff_algo::BackoffAlgorithm::new(8, 64, random());
        println!("{}", bfa.get());
        assert!(bfa.next() <= Some(16));
        println!("{}", bfa.get());
        assert!(bfa.next() <= Some(32));
        println!("{}", bfa.get());
        assert!(bfa.next() <= Some(64));
        println!("{}", bfa.get());
    }
}
