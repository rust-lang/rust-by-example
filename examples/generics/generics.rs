// A concrete type `T`.
struct T;

// The first use of `T` was not preceded by `<T>` so `Single` must
// be a concrete type. `T` is defined at the top.
struct Single(T);
//            ^ Here is `Single`s first use of the type `T`.

// The first use of `T` is preceded by `<T>`. `SingleGen` must be
// generic and has not yet been specialized. `T` could be anything
// including `T` at the top.
struct SingleGen<T>(T);

// Instantiating the types can be implicit or explicit.
fn main() {
    // Regular `Single`.
    let _s = Single(T);

    // `SingleGen` explicity specialized.
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen`s implicitly specialized.
    let _t   = SingleGen(T); // Uses `T` at top.
    let _i32 = SingleGen(6); // Uses `i32`.
}
