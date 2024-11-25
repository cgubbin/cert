//! Defines the [`Uncertainty`] trait.
//!
//! The uncertainty trait defines common behaviours for quantities with an associated uncertainty.

use super::{AbsUncertainty, RelUncertainty};
use num_traits::{Float, One, Zero};
use std::ops;

/// Used to define behaviour for values which have associated uncertainty.
pub trait Uncertainty:
    Sized
    + Copy
    + Into<AbsUncertainty<Self::Float>>
    + Into<RelUncertainty<Self::Float>>
    + ops::Add<Self, Output = Self>
    + ops::Div<Self, Output = Self>
    + ops::Mul<Self, Output = Self>
    + ops::Sub<Self, Output = Self>
    + Zero
{
    /// The underlying float type for the uncertainty
    type Float: Float + Zero;

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
            ((Self::Float::one() + Self::Float::one()).powi(n)
                * self.mean().powi(2 * n - 2)
                * self.uncertainty().powi(2))
            .sqrt(),
        )
    }
}

impl Uncertainty for f64 {
    type Float = f64;

    fn new(value: f64, _uncertainty: f64) -> f64 {
        value
    }

    fn mean(&self) -> Self::Float {
        *self
    }

    fn standard_deviation(&self) -> Self::Float {
        0.0
    }

    fn coefficient_of_variation(&self) -> Self::Float {
        0.0
    }

    fn uncertainty(&self) -> Self::Float {
        0.0
    }

    fn is_certain(&self) -> bool {
        true
    }

    fn powi(&self, n: i32) -> Self {
        <f64 as Float>::powi(*self, n)
    }
}

impl From<f64> for AbsUncertainty<f64> {
    fn from(val: f64) -> AbsUncertainty<f64> {
        AbsUncertainty::new(val, 0.0)
    }
}

impl From<f64> for RelUncertainty<f64> {
    fn from(val: f64) -> RelUncertainty<f64> {
        RelUncertainty::new(val, 0.0)
    }
}

impl Uncertainty for f32 {
    type Float = f32;

    fn new(value: f32, _uncertainty: f32) -> f32 {
        value
    }

    fn mean(&self) -> Self::Float {
        *self
    }

    fn standard_deviation(&self) -> Self::Float {
        0.0
    }

    fn coefficient_of_variation(&self) -> Self::Float {
        0.0
    }

    fn uncertainty(&self) -> Self::Float {
        0.0
    }

    fn is_certain(&self) -> bool {
        true
    }

    fn powi(&self, n: i32) -> Self {
        <f32 as Float>::powi(*self, n)
    }
}

impl From<f32> for AbsUncertainty<f32> {
    fn from(val: f32) -> AbsUncertainty<f32> {
        AbsUncertainty::new(val, 0.0)
    }
}

impl From<f32> for RelUncertainty<f32> {
    fn from(val: f32) -> RelUncertainty<f32> {
        RelUncertainty::new(val, 0.0)
    }
}

impl From<AbsUncertainty<f32>> for f32 {
    fn from(val: AbsUncertainty<f32>) -> f32 {
        val.mean()
    }
}

impl From<AbsUncertainty<f64>> for f64 {
    fn from(val: AbsUncertainty<f64>) -> f64 {
        val.mean()
    }
}
