use super::{AbsUncertainty, Uncertainty};
use num_traits::{Float, FromPrimitive, Zero};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(test, derive(PartialEq))]
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

impl<N: Default + Float + FromPrimitive + Zero> Zero for RelUncertainty<N> {
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

impl<N: Default + Float + FromPrimitive> Into<AbsUncertainty<N>> for RelUncertainty<N> {
    fn into(self: RelUncertainty<N>) -> AbsUncertainty<N> {
        AbsUncertainty::new(self.mean, self.coefficient_of_variation * self.mean)
    }
}

impl<N: Default + Float + FromPrimitive> Uncertainty for RelUncertainty<N> {
    type Float = N;

    fn new(mean: N, coefficient_of_variation: N) -> RelUncertainty<N> {
        RelUncertainty {
            mean,
            coefficient_of_variation,
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

impl<N: Float + FromPrimitive + fmt::Display> fmt::Display for RelUncertainty<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:2} ± {:2}",
            self.mean,
            self.coefficient_of_variation * N::from_f64(100.0).unwrap()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::RelUncertainty;

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
}
