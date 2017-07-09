fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // Результат этого выражение будет присвоен переменной `y`
        x_cube + x_squared + x
    };

    let z = {
        // Т.к это выражение оканчивается на `;`, переменной `z` будет присвоен `()`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
