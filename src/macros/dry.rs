#![feature(macro_rules)]

macro_rules! assert_len {
    ($func:ident, $op:tt) => {
        assert!(xs.len() == ys.len(),
                "{}: dimension mismatch: {} {} {}",
                stringify!($func),
                (xs.len(),),
                stringify!($op),
                (ys.len(),));
    }
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, T>>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_len!($func, $op);

            for (x, y) in xs.mut_iter().zip(ys.iter()) {
                *x = x.$method(y);
            }
        }
    }
}

// implement add_assign, mul_assign, and sub_assign functions
op!(add_assign, Add, +=, add)
op!(mul_assign, Mul, *=, mul)
op!(sub_assign, Sub, -=, sub)

fn main() {
    let mut xs = Vec::from_elem(5, 0.0);
    let ys = Vec::from_elem(6, 1.0);

    // this operation will fail at runtime
    add_assign(&mut xs, &ys);
}

mod test {
    macro_rules! test {
        ($func: ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $func() {
                for size in range(0u, 10) {
                    let mut x = Vec::from_elem(size, $x);
                    let y = Vec::from_elem(size, $y);
                    let z = Vec::from_elem(size, $z);

                    super::$func(&mut x, &y);

                    assert_eq!(x, z);
                }
            }
        }
    }

    // test add_assign, mul_assign and sub_assign
    test!(add_assign, 1, 2, 3)
    test!(mul_assign, 2, 3, 6)
    test!(sub_assign, 3, 2, 1)
}
