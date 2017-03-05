use std::mem;

// Эта функция заимствует срез
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Массив фиксированного размера (указывать сигнатуру типа необязательно)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // Все элементы могут быть инициализированы одной и той же переменной
    let ys: [i32; 500] = [0; 500];

    // Индекс начинается с 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` возвращает длину массива
    println!("array size: {}", xs.len());

    // Память для массивов выделяется в стеке
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Массивы могут быть автоматически заимствованы как срез
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Срезы могут указывать на часть массива
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // Выход за границу массива заставит компилятор паниковать.
    // Не надо так.
    println!("{}", xs[5]);
}
