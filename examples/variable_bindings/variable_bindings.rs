fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // скопировать значение `an_integer` в `copied_integer`
    let copied_integer = an_integer;

    println!("Целое: {:?}", copied_integer);
    println!("Логическое: {:?}", a_boolean);
    println!("Встречайте единичное значение: {:?}", unit);

    // Компилятор предупреждает о неиспользуемых переменных; эти предупреждения можно
    // отключить используя подчёркивание перед именем переменной
    let _unused_variable = 3u32;
    let noisy_unused_variable = 2u32;
    // ИСПРАВЬТЕ ^ Добавьте подчёркивание
}
