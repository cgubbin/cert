Cert
---

Cert is a simple library for handling and manipulating numerical values associated with an uncertainty. It provides a common interface for handling types with relative and absolute uncertainty, and facilitates conversion between the two. It also implements common mathematical operations on types implementing the `Uncertainty` trait. This allows the values to be operated on, respecting propagation of error into the final result.

Cert exposes two concrete types, a `AbsUncertainty` and a `RelUncertainty`, both of which implement the `Uncertainty` interface. The `AbsUncertainty` type is used to represent absolute uncertainties, or values where the uncertainty is defined by the standard deviation of the value. The `RelUncertainty` type is used to represent relative uncertainties, where the uncertainty level is defined by the coefficient of variation, or the ratio of the standard deviation and the value.

Both types are generic over the inner float, which has to implement the `Float` and `Zero` traits from the `num_traits` crate.
