use super::{AbsUncertainty, RelUncertainty};
use num_traits::{Float, FromPrimitive};

/// Used to define behaviour for values which have uncertain values.
pub trait Uncertainty:
    Sized
    + Copy
    + Default
    + Into<AbsUncertainty<Self::Float>>
    + Into<RelUncertainty<Self::Float>>
    + std::ops::Add<Self, Output = Self>
    + std::ops::Div<Self, Output = Self>
    + std::ops::Mul<Self, Output = Self>
    + std::ops::Sub<Self, Output = Self>
    + num_traits::Zero
{
    type Float: Float + FromPrimitive;

    fn new(value: Self::Float, uncertainty: Self::Float) -> Self;

    /// Get the base value of the uncertainty.
    fn value(&self) -> Self::Float;
    /// Get the uncertainty value; this will depend on whether the base type is a relative or
    /// absolute uncertainty.
    fn uncertainty(&self) -> Self::Float;

    fn abs_uncertainty(&self) -> Self::Float;

    fn powi(&self, n: i32) -> Self {
        Self::new(
            self.value().powi(n),
            (<Self::Float as FromPrimitive>::from_i32(n).unwrap().powi(2)
                * self.value().powi(2 * n - 2)
                * self.uncertainty().powi(2))
            .sqrt(),
        )
    }
}
