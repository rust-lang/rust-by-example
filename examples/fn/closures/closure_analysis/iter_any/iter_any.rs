fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter` yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    // `into_iter` yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` is normal. `into_iter()` for arrays unusually
    // yields `&i32`. These would not normally both be `&`.
    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}
