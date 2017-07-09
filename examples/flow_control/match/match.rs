fn main() {
    let number = 13;
    // ЗАДАНИЕ ^ Попробуйте присвоить `number` другое значение

    println!("Tell me about {}", number);
    match number {
        // Сопоставление с одним значением
        1 => println!("One!"),
        // Сопоставление с несколькими значениями
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Сопоставление с диапазоном значений
        13...19 => println!("A teen"),
        // Обработка остальных случаев
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // Match так же является выражением
    let binary = match boolean {
        // Ветви match должны обработать все возможные значения переменной
        false => 0,
        true => 1,
        // ЗАДАНИЕ ^ Попробуйте закомментировать эту ветвь
    };

    println!("{} -> {}", boolean, binary);
}
