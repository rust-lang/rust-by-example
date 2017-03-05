fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Перед изменением: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("После изменения: {}", mutable_binding);

    // Ошибка!
    _immutable_binding += 1;
    // ИСПРАВЬТЕ ^ Закомментируйте эту строку
}
