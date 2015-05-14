struct T;          // Concrete type.
struct S(T);       // Concrete type.
struct SGen<T>(T); // Generic type.

// These functions all take ownership of the variable passed into
// them and immediately go out of scope freeing the variable.
//
// This has no preceding `<T>` so this must be a regular function.
fn die_regular(s: S) {}

// Has a `<T>` but it isn't preceded by `<T>` to make it generic.
// This is a regular function which takes `SGen<T>` which has
// been specialized to type `T` defined at the top.
fn die_generic_specialized_t(s: SGen<T>) {}

// A regular function taking `SGen<T>` specialized to `i32`.
fn die_generic_specialized_i32(s: SGen<i32>) {}

// `<T>` is preceded by `<T>`. This function is generic over `T`.
fn die_generic<T>(s: SGen<T>) {}

fn main() {
    // Use the regular functions like normal
    die_regular(S(T));                    // Concrete type.
    die_generic_specialized_t(SGen(T));   // Specialized generic type.
    die_generic_specialized_i32(SGen(6)); // Specialized generic type.

    // Explicitly specialize `die_generic()` to `char`.
    die_generic::<char>(SGen('a'));

    // Implicitly specialize `die_generic()` to `char`.
    die_generic(SGen('c'));
}

