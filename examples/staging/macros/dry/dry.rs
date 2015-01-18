use std::iter;
use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    // The `tt` (token tree) designator is used for
    // operators and tokens
    ($a:ident, $b: ident, $func:ident, $op:tt) => (
        assert!($a.len() == $b.len(),
                "{:?}: dimension mismatch: {:?} {:?} {:?}",
                stringify!($func),
                ($a.len(),),
                stringify!($op),
                ($b.len(),));
    )
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => (
        fn $func<T: $bound<T, Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
                // *x = x.$method(*y);
            }
        }
    )
}

// implement add_assign, mul_assign, and sub_assign functions
op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

fn main() {
    let mut xs = iter::repeat(0f64).take(5).collect();
    let ys = iter::repeat(1f64).take(6).collect();

    // this operation will fail at runtime
    add_assign(&mut xs, &ys);
}

mod test {
    use std::iter;
    macro_rules! test {
        ($func: ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $func() {
                for size in range(0u32, 10) {
                    let mut x: Vec<_> = iter::repeat(size).take($x).collect();
                    let y: Vec<_> = iter::repeat(size).take($y).collect();
                    let z: Vec<_> = iter::repeat(size).take($z).collect();

                    super::$func(&mut x, &y);

                    assert_eq!(x, z);
                }
            }
        }
    }

    // test add_assign, mul_assign and sub_assign
    test!(add_assign, 1us, 2us, 3us);
    test!(mul_assign, 2us, 3us, 6us);
    test!(sub_assign, 3us, 2us, 1us);
}

