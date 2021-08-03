use std::cmp::Ordering;

/// The minimum and maximum sequence numbers seen for a given sequencer.
///
/// **IMPORTANT: These ranges include their start and their end (aka `[start, end]`)!**
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MinMaxSequence {
    min: u64,
    max: u64,
}

impl MinMaxSequence {
    /// Create new min-max sequence range.
    ///
    /// This panics if `min > max`.
    pub fn new(min: u64, max: u64) -> Self {
        assert!(
            min <= max,
            "min ({}) is greater than max ({}) sequence",
            min,
            max
        );
        Self { min, max }
    }

    pub fn min(&self) -> u64 {
        self.min
    }

    pub fn max(&self) -> u64 {
        self.max
    }
}

/// The optional minimum and maximum sequence numbers seen for a given sequencer.
///
/// **IMPORTANT: These ranges include their start and their end (aka `[start, end]`)!**
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OptionalMinMaxSequence {
    min: Option<u64>,
    max: u64,
}

impl OptionalMinMaxSequence {
    /// Create new min-max sequence range.
    ///
    /// This panics if `min > max`.
    pub fn new(min: Option<u64>, max: u64) -> Self {
        if let Some(min) = min {
            assert!(
                min <= max,
                "min ({}) is greater than max ({}) sequence",
                min,
                max
            );
        }
        Self { min, max }
    }

    pub fn min(&self) -> Option<u64> {
        self.min
    }

    pub fn max(&self) -> u64 {
        self.max
    }

    /// Compares range the other.
    ///
    /// Returns `None` if ranges cannot be compared.
    pub fn try_cmp(&self, other: &Self) -> Option<Ordering> {
        let min1 = self
            .min
            .map(Some)
            .unwrap_or_else(|| self.max.checked_add(1));
        let min2 = other
            .min
            .map(Some)
            .unwrap_or_else(|| other.max.checked_add(1));

        let cmp_min = match (min1, min2) {
            (Some(min1), Some(min2)) => {
                // no overflow
                min1.cmp(&min2)
            }
            (Some(_), None) => {
                // min2 overflowed and is greater
                Ordering::Less
            }
            (None, Some(_)) => {
                // min1 overflowed and is greater
                Ordering::Greater
            }
            (None, None) => {
                // both overflowed and are equal
                Ordering::Equal
            }
        };

        match (cmp_min, self.max.cmp(&other.max)) {
            (Ordering::Equal, x) => Some(x),
            (x, Ordering::Equal) => Some(x),
            (x, y) if x == y => Some(x),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_max_getters() {
        let min_max = MinMaxSequence::new(10, 20);
        assert_eq!(min_max.min(), 10);
        assert_eq!(min_max.max(), 20);
    }

    #[test]
    fn test_opt_min_max_getters() {
        let min_max = OptionalMinMaxSequence::new(Some(10), 20);
        assert_eq!(min_max.min(), Some(10));
        assert_eq!(min_max.max(), 20);

        let min_max = OptionalMinMaxSequence::new(None, 20);
        assert_eq!(min_max.min(), None);
        assert_eq!(min_max.max(), 20);
    }

    #[test]
    fn test_min_max_accepts_equal_values() {
        MinMaxSequence::new(10, 10);
    }

    #[test]
    fn test_opt_min_max_accepts_equal_values() {
        OptionalMinMaxSequence::new(Some(10), 10);
    }

    #[test]
    #[should_panic(expected = "min (11) is greater than max (10) sequence")]
    fn test_min_max_checks_values() {
        MinMaxSequence::new(11, 10);
    }

    #[test]
    #[should_panic(expected = "min (11) is greater than max (10) sequence")]
    fn test_opt_min_max_checks_values() {
        OptionalMinMaxSequence::new(Some(11), 10);
    }

    #[test]
    fn test_opt_min_max_try_cmp() {
        assert_eq!(
            OptionalMinMaxSequence::new(Some(10), 10)
                .try_cmp(&OptionalMinMaxSequence::new(Some(10), 10)),
            Some(Ordering::Equal),
        );

        assert_eq!(
            OptionalMinMaxSequence::new(None, 10)
                .try_cmp(&OptionalMinMaxSequence::new(Some(10), 10)),
            Some(Ordering::Greater),
        );
        assert_eq!(
            OptionalMinMaxSequence::new(Some(10), 10)
                .try_cmp(&OptionalMinMaxSequence::new(None, 10)),
            Some(Ordering::Less),
        );

        assert_eq!(
            OptionalMinMaxSequence::new(None, 10)
                .try_cmp(&OptionalMinMaxSequence::new(Some(11), 11)),
            Some(Ordering::Less),
        );

        assert_eq!(
            OptionalMinMaxSequence::new(Some(10), 10)
                .try_cmp(&OptionalMinMaxSequence::new(Some(9), 11)),
            None,
        );
        assert_eq!(
            OptionalMinMaxSequence::new(None, 10)
                .try_cmp(&OptionalMinMaxSequence::new(Some(9), 11)),
            None,
        );

        assert_eq!(
            OptionalMinMaxSequence::new(Some(u64::MAX), u64::MAX)
                .try_cmp(&OptionalMinMaxSequence::new(Some(u64::MAX), u64::MAX)),
            Some(Ordering::Equal),
        );
        assert_eq!(
            OptionalMinMaxSequence::new(None, u64::MAX)
                .try_cmp(&OptionalMinMaxSequence::new(None, u64::MAX)),
            Some(Ordering::Equal),
        );
        assert_eq!(
            OptionalMinMaxSequence::new(None, u64::MAX)
                .try_cmp(&OptionalMinMaxSequence::new(Some(u64::MAX), u64::MAX)),
            Some(Ordering::Greater),
        );
        assert_eq!(
            OptionalMinMaxSequence::new(Some(u64::MAX), u64::MAX)
                .try_cmp(&OptionalMinMaxSequence::new(None, u64::MAX)),
            Some(Ordering::Less),
        );
    }
}
