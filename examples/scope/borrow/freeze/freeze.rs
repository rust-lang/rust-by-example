fn main() {
    let mut _mutable_integer = 7i32;

    {
        // Заимствовать `_mutable_integer`
        let _large_integer = &_mutable_integer;

        // Ошибка! `_mutable_integer` заморожен в этой области видимости
        _mutable_integer = 50;
        // ИСПРАВЬТЕ ^ Закомментируйте эту строку

        // `_large_integer` покидает область видимости
    }

    // Ок! `_mutable_integer` не заморожен в этой области видимости
    _mutable_integer = 3;
}
