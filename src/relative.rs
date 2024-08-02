use super::{AbsUncertainty, Uncertainty};
use num_traits::{Float, FromPrimitive, Zero};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RelUncertainty<N> {
    value: N,
    uncertainty: N,
}

impl<N: Default + Float + FromPrimitive + Zero> Zero for RelUncertainty<N> {
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

impl<N: Default + Float + FromPrimitive> Into<AbsUncertainty<N>> for RelUncertainty<N> {
    fn into(self: RelUncertainty<N>) -> AbsUncertainty<N> {
        AbsUncertainty::new(self.value, self.uncertainty * self.value)
    }
}

impl<N: Default + Float + FromPrimitive> Uncertainty for RelUncertainty<N> {
    type Float = N;

    fn new(value: N, uncertainty: N) -> RelUncertainty<N> {
        RelUncertainty { value, uncertainty }
    }

    fn value(&self) -> N {
        self.value
    }

    fn uncertainty(&self) -> N {
        self.uncertainty
    }

    fn abs_uncertainty(&self) -> N {
        self.uncertainty * self.value
    }
}

impl<N: Float + FromPrimitive + fmt::Display> fmt::Display for RelUncertainty<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ± {}%",
            self.value,
            self.uncertainty * N::from_f64(100.0).unwrap()
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
