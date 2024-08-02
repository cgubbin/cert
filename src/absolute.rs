use super::{RelUncertainty, Uncertainty};
use num_traits::{Float, FromPrimitive, Zero};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(test, derive(PartialEq))]
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

impl<N: Default + Float + FromPrimitive + Zero> Zero for AbsUncertainty<N> {
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

impl<N: Default + Float + FromPrimitive> Into<RelUncertainty<N>> for AbsUncertainty<N> {
    fn into(self: AbsUncertainty<N>) -> RelUncertainty<N> {
        RelUncertainty::new(self.mean, self.standard_deviation / self.mean)
    }
}

impl<N: Default + Float + FromPrimitive> Uncertainty for AbsUncertainty<N> {
    type Float = N;

    fn new(mean: N, standard_deviation: N) -> AbsUncertainty<N> {
        AbsUncertainty {
            mean,
            standard_deviation,
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
    use super::AbsUncertainty;

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
}
