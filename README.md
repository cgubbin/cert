Surety
---

Surety is a simple library for handling and manipulating numerical values associated with an uncertainty. 

Surety exposes two concrete types, an `AbsUncertainty` and a `RelUncertainty`, both of which implement the `Uncertainty` interface. The `AbsUncertainty` type is used to represent absolute uncertainties, or values where the uncertainty is defined by the standard deviation of the value. The `RelUncertainty` type is used to represent relative uncertainties, where the uncertainty level is defined by the coefficient of variation, or the ratio of the standard deviation and the value.

Both types are generic over the inner float, which has to implement the `Float` and `Zero` traits from the `num_traits` crate.
