//! Relative uncertainties.
//!
//! Relative uncertainties are characterized by a mean value and a coefficient of variation. This
//! describes the values standard deviation as a fraction of the mean value.

use super::{AbsUncertainty, Uncertainty};
use num_traits::{Float, Zero};
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// A relative uncertainty.
///
/// Relative uncertainties are characterized by a mean value and a coefficient of variation. The
/// latter is the ratio of the standard deviation to the mean value.
pub struct RelUncertainty<N> {
    /// Central value of the uncertainty.
    mean: N,
    /// Coefficient of variation of the uncertainty.
    ///
    /// The coefficient of variation is stored as a fractional quantity.
    coefficient_of_variation: N,
}

impl<N: Float + Zero + fmt::Display> fmt::Display for RelUncertainty<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} ± {:.2}", self.mean, self.standard_deviation())
    }
}

impl<N: Float + Zero> Zero for RelUncertainty<N> {
    fn zero() -> Self {
        Self {
            mean: N::zero(),
            coefficient_of_variation: N::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.mean.is_zero()
    }
}

impl<N: Float + Zero> From<RelUncertainty<N>> for AbsUncertainty<N> {
    fn from(val: RelUncertainty<N>) -> AbsUncertainty<N> {
        AbsUncertainty::new(val.mean, val.coefficient_of_variation * val.mean)
    }
}

impl<N: Float + Zero> Uncertainty for RelUncertainty<N> {
    type Float = N;

    fn new(mean: N, coefficient_of_variation: N) -> RelUncertainty<N> {
        RelUncertainty {
            mean,
            coefficient_of_variation: coefficient_of_variation.abs(),
        }
    }

    fn mean(&self) -> N {
        self.mean
    }

    fn standard_deviation(&self) -> N {
        self.mean * self.coefficient_of_variation
    }

    fn coefficient_of_variation(&self) -> N {
        self.coefficient_of_variation
    }

    fn uncertainty(&self) -> N {
        self.coefficient_of_variation
    }
}

#[cfg(test)]
mod tests {
    use super::{RelUncertainty, Uncertainty};
    use crate::AbsUncertainty;
    use approx::assert_relative_eq;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<RelUncertainty<f64>>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<RelUncertainty<f64>>();
    }

    #[test]
    fn test_relative_round_trip() {
        let mut rng = thread_rng();
        let relative = RelUncertainty::<f64>::new(rng.gen(), rng.gen());

        let absolute = AbsUncertainty::from(relative);
        let result = RelUncertainty::from(absolute);

        assert_relative_eq!(relative.mean(), result.mean());
        assert_relative_eq!(relative.standard_deviation(), result.standard_deviation());
    }
}
