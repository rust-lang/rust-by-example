The `std::rand` module provides access to the Random Number Generators (RNG)
provided by the OS, which can then be used to generate random values of any
type that implements the `Rand` trait (which includes all the primitive types).

{gen.rs}

{gen.out}

Several structs are available under the `std::rand::distributions` module, that
can be used to generate values using different probability distributions like:
normal, uniform, Student's T, Chi squared, Gamma, etc.

{distributions.rs}

{distributions.out}

The `derive` attribute can be used to implement the `Rand` trait for custom
types, this allows generation of random values of custom types.

{derive.rs}

{derive.out}
