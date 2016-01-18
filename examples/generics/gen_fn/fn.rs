struct A;          // Concrete type `A`.
struct S(A);       // Concrete type `S`.
struct SGen<T>(T); // Generic type `SGen`.

// These functions all take ownership of the variable passed into
// them and immediately go out of scope freeing the variable.

// Define a function `reg_fn` that takes an argument `s` of type `S`.
// This has no `<T>` so this is not a generic function.
fn reg_fn(s: S) {}

// Define a function `gen_spec_t` that takes an argument `s` of type `SGen<T>`
// that has been explicitly given the type parameter `A`.
// This contains `<A>` but is not preceded by `<A>`, so it is not generic.
fn gen_spec_t(s: SGen<A>) {}

// Define a function `gen_spec_i32` that takes an argument `s` of type `SGen<i32>`
// that has been explicitly given the type parameter `i32`.
// This function is also not generic.
fn gen_spec_i32(s: SGen<i32>) {}

// Define a function `generic` that takes an argument `s` of type `SGen<T>`.
// Because `SGen<T>` is preceded by `<T>`, this function is generic over `T`.
fn generic<T>(s: SGen<T>) {}

fn main() {
    // Using the non-generic functions
    reg_fn(S(A));          // Concrete type.
    gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(6)); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('a'));

    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen('c'));
}

