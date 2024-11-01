//! Absolute uncertainties.
//!
//! Absolute uncertainties are characterized by a mean value and a standard deviation

use super::{RelUncertainty, Uncertainty};
use num_traits::{Float, Zero};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// An absolute uncertainty.
///
/// Absolute uncertainties are values that are characterized by a mean, and a standard deviation.
pub struct AbsUncertainty<N> {
    /// The central estimate of the underlying quantity.
    mean: N,
    /// The standard deviation of the underlying quantity.
    standard_deviation: N,
}

impl<N: fmt::Display> fmt::Display for AbsUncertainty<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} ± {:.2}", self.mean, self.standard_deviation)
    }
}

impl<N: Float + Zero> Zero for AbsUncertainty<N> {
    fn zero() -> Self {
        Self {
            mean: N::zero(),
            standard_deviation: N::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.mean.is_zero()
    }
}

impl<N: Float> From<AbsUncertainty<N>> for RelUncertainty<N> {
    fn from(val: AbsUncertainty<N>) -> RelUncertainty<N> {
        RelUncertainty::new(val.mean, val.standard_deviation / val.mean)
    }
}

impl<N: Float + Zero> Uncertainty for AbsUncertainty<N> {
    type Float = N;

    fn new(mean: N, standard_deviation: N) -> AbsUncertainty<N> {
        AbsUncertainty {
            mean,
            standard_deviation: standard_deviation.abs(),
        }
    }

    fn mean(&self) -> N {
        self.mean
    }

    fn standard_deviation(&self) -> N {
        self.standard_deviation
    }

    fn coefficient_of_variation(&self) -> N {
        self.standard_deviation / self.mean
    }

    // The uncertainty is the standard deviation for an absolute uncertainty.
    fn uncertainty(&self) -> N {
        self.standard_deviation()
    }
}

#[cfg(test)]
mod tests {
    use super::{AbsUncertainty, Uncertainty};
    use crate::RelUncertainty;
    use approx::assert_relative_eq;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<AbsUncertainty<f64>>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<AbsUncertainty<f64>>();
    }

    #[test]
    fn test_absolute_round_trip() {
        let mut rng = thread_rng();
        let absolute = AbsUncertainty::<f64>::new(rng.gen(), rng.gen());

        let relative = RelUncertainty::from(absolute);
        let result = AbsUncertainty::from(relative);

        assert_relative_eq!(absolute.mean(), result.mean());
        assert_relative_eq!(absolute.standard_deviation(), result.standard_deviation());
    }
}
