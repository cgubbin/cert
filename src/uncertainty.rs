use super::{AbsUncertainty, RelUncertainty};
use num_traits::{Float, FromPrimitive, Zero};
use std::ops;

/// Used to define behaviour for values which have uncertain values.
pub trait Uncertainty:
    Sized
    + Copy
    + Default
    + Into<AbsUncertainty<Self::Float>>
    + Into<RelUncertainty<Self::Float>>
    + ops::Add<Self, Output = Self>
    + ops::Div<Self, Output = Self>
    + ops::Mul<Self, Output = Self>
    + ops::Sub<Self, Output = Self>
    + num_traits::Zero
{
    /// The underlying float type for the uncertainty
    type Float: Float + FromPrimitive + Zero;

    fn new(value: Self::Float, uncertainty: Self::Float) -> Self;

    /// Returns the mean of the value
    fn mean(&self) -> Self::Float;
    /// The standard deviation of the value
    fn standard_deviation(&self) -> Self::Float;
    /// The coefficient of variation is the ratio of the standard deviation and the mean.
    ///
    /// It describes the relative error.
    fn coefficient_of_variation(&self) -> Self::Float;

    /// The actual uncertainty of the concrete type. This will depend on whether the implementor
    /// represent a relative or absolute uncertainty.
    fn uncertainty(&self) -> Self::Float;

    /// Returns true if the uncertainty is zero.
    ///
    /// This method is useful when the type may be used in a method which divides by the
    /// uncertainties.
    fn is_certain(&self) -> bool {
        self.uncertainty() == Self::Float::zero()
    }

    /// Raise the value to the nth power.
    fn powi(&self, n: i32) -> Self {
        Self::new(
            self.mean().powi(n),
            (<Self::Float as FromPrimitive>::from_i32(n).unwrap().powi(2)
                * self.mean().powi(2 * n - 2)
                * self.uncertainty().powi(2))
            .sqrt(),
        )
    }
}
