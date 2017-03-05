fn main() {
    // Переменные могут быть аннотированы.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Обычная аннотация
    let an_integer   = 5i32; // Суффиксная аннотация

    // Этим переменным будет присвоен тип по умолчанию.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    let mut mutable = 12; // Изменяемый `i32`.

    // Ошибка! Тип переменной не может быть изменён!
    mutable = true;
}
