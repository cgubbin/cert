use super::{AbsUncertainty, RelUncertainty, Uncertainty};
use num_traits::{Float, FromPrimitive};
use std::ops::{Add, Div, Mul, Sub};

impl<N: Default + Float + FromPrimitive, U: Uncertainty<Float = N>> Add<U> for RelUncertainty<N> {
    type Output = RelUncertainty<N>;
    fn add(self, other: U) -> Self {
        let other: RelUncertainty<N> = other.into();
        Self::new(
            self.value() + other.value(),
            (self.uncertainty().powi(2) + other.uncertainty().powi(2)).sqrt(),
        )
    }
}

impl<N: Default + Float + FromPrimitive, U: Uncertainty<Float = N>> Add<U> for AbsUncertainty<N> {
    type Output = AbsUncertainty<N>;
    fn add(self, other: U) -> Self {
        let other: AbsUncertainty<N> = other.into();
        Self::new(
            self.value() + other.value(),
            (self.uncertainty().powi(2) + other.uncertainty().powi(2)).sqrt(),
        )
    }
}

impl<N: Default + Float + FromPrimitive, U: Uncertainty<Float = N>> Sub<U> for RelUncertainty<N> {
    type Output = RelUncertainty<N>;
    fn sub(self, other: U) -> Self {
        let other: RelUncertainty<N> = other.into();
        Self::new(
            self.value() - other.value(),
            (self.uncertainty().powi(2) + other.uncertainty().powi(2)).sqrt(),
        )
    }
}

impl<N: Default + Float + FromPrimitive, U: Uncertainty<Float = N>> Sub<U> for AbsUncertainty<N> {
    type Output = AbsUncertainty<N>;
    fn sub(self, other: U) -> Self {
        let other: AbsUncertainty<N> = other.into();
        Self::new(
            self.value() - other.value(),
            (self.uncertainty().powi(2) + other.uncertainty().powi(2)).sqrt(),
        )
    }
}

impl<N: Default + Float + FromPrimitive, U: Uncertainty<Float = N>> Mul<U> for RelUncertainty<N> {
    type Output = RelUncertainty<N>;
    fn mul(self, other: U) -> Self {
        let other: RelUncertainty<N> = other.into();
        Self::new(
            self.value() * other.value(),
            (self.uncertainty().powi(2) * other.value().powi(2)
                + other.uncertainty().powi(2) * self.value().powi(2))
            .sqrt(),
        )
    }
}

impl<N: Default + Float + FromPrimitive, U: Uncertainty<Float = N>> Mul<U> for AbsUncertainty<N> {
    type Output = AbsUncertainty<N>;
    fn mul(self, other: U) -> Self {
        let other: AbsUncertainty<N> = other.into();
        Self::new(
            self.value() * other.value(),
            (self.uncertainty().powi(2) * other.value().powi(2)
                + other.uncertainty().powi(2) * self.value().powi(2))
            .sqrt(),
        )
    }
}

impl<N: Default + Float + FromPrimitive, U: Uncertainty<Float = N>> Div<U> for RelUncertainty<N> {
    type Output = RelUncertainty<N>;
    fn div(self, other: U) -> Self {
        let other: RelUncertainty<N> = other.into();
        Self::new(
            self.value() / other.value(),
            (self.uncertainty().powi(2) / other.value().powi(2)
                + self.value().powi(2)
                    * (other.uncertainty() * other.value().powi(2)).abs().powi(2))
            .sqrt(),
        )
    }
}

impl<N: Default + Float + FromPrimitive, U: Uncertainty<Float = N>> Div<U> for AbsUncertainty<N> {
    type Output = AbsUncertainty<N>;
    fn div(self, other: U) -> Self {
        let other: AbsUncertainty<N> = other.into();
        Self::new(
            self.value() / other.value(),
            (self.uncertainty().powi(2) / other.value().powi(2)
                + self.value().powi(2)
                    * (other.uncertainty() * other.value().powi(2)).abs().powi(2))
            .sqrt(),
        )
    }
}
