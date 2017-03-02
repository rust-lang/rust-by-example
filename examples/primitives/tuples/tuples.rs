// Кортежи могут быть использованы как аргументы функции
// и как возвращаемые значения
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` можно использовать для создания связи между кортежем и переменной
    let (integer, boolean) = pair;

    (boolean, integer)
}

// Это структура используется для задания
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // Кортеж с множеством различных типов данных
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // К значениям переменных внутри кортежа можно обратиться по индексу
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Кортежи могут содержать в себе кортежи
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Кортежи можно напечатать
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // Для создания кортежа, содержащего один элемент, необходимо написать элемент и
    // поставить запятую внутри круглых скобок.
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // Кортежи можно разобрать на части (деструктурировать) для создания связи
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix)

}
