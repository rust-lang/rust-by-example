fn main() {
    // Литералы с суффиксами. Их тип известен при инициализации.
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Литералы без суффиксов. Их тип будет зависеть от того, как их используют.
    let i = 1;
    let f = 1.0;

    // `size_of_val` возвращает размер переменной в байтах
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}
