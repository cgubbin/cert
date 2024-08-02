use super::{RelUncertainty, Uncertainty};
use num_traits::{Float, FromPrimitive, Zero};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AbsUncertainty<N> {
    value: N,
    uncertainty: N,
}

impl<N: Default + Float + FromPrimitive + Zero> Zero for AbsUncertainty<N> {
    fn zero() -> Self {
        Self {
            value: N::zero(),
            uncertainty: N::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl<N: Default + Float + FromPrimitive> Into<RelUncertainty<N>> for AbsUncertainty<N> {
    fn into(self: AbsUncertainty<N>) -> RelUncertainty<N> {
        RelUncertainty::new(self.value, self.uncertainty / self.value)
    }
}

impl<N: Default + Float + FromPrimitive> Uncertainty for AbsUncertainty<N> {
    type Float = N;

    fn new(value: N, uncertainty: N) -> AbsUncertainty<N> {
        AbsUncertainty { value, uncertainty }
    }

    fn value(&self) -> N {
        self.value
    }

    fn uncertainty(&self) -> N {
        self.uncertainty
    }

    fn abs_uncertainty(&self) -> N {
        self.uncertainty
    }
}

impl<N: fmt::Display> fmt::Display for AbsUncertainty<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ± {}", self.value, self.uncertainty)
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
