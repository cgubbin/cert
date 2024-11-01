//! Cert provides abstractions for representing numerical quantities with associated
//! uncertainties.
//!
//! It provides two concrete types for representing uncertainties, both characterized by
//! two quantities: a value and an uncertainty. For both the value represents the mean of the
//! quantity. For the [`AbsUncertainty`] type the uncertainty is the standard deviation, while for
//! the [`RelUncertainty`] type the uncertainty is the coefficient of variation, or the ratio of
//! the standard deviation and the mean.
//!
//! # Examples
//! ```
//! use cert::{AbsUncertainty, RelUncertainty, Uncertainty};
//!
//! let absolute_uncertainty = AbsUncertainty::new(10.0, 1.0);
//! let relative_uncertainty = RelUncertainty::from(absolute_uncertainty);
//! ```

/// Absolute uncertainties
///
/// An [`AbsUncertainty`] represents a quantity with an associated standard deviation, defined in
/// the same unit as the quantity itself.
mod absolute;

/// Implementation of common operations on the uncertainty types
mod ops;

/// Relative uncertainties
///
/// An [`RelUncertainty`] represents a quantity with an associated standard deviation, defined as
/// a fraction of the quantity itself.
mod relative;

/// The generic [`Uncertaintly`] trait, implemented by both [`AbsUncertainty`] and [`RelUncertainty`]
mod uncertainty;

pub use absolute::AbsUncertainty;
pub use relative::RelUncertainty;
pub use uncertainty::Uncertainty;
