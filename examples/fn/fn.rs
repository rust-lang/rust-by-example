// В отличие от С/С++, нет никаких ограничений касаемо порядка определений функций
fn main() {
    // Можно использовать функцию здесь, а определить где-нибудь потом
    fizzbuzz_to(100);
}

// Функция, возвращающая логическое значение
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Граничный случай, ранний возврат
    if rhs == 0 {
        return false;
    }

    // Это - выражение, ключевое слово `return` здесь не требуется
    lhs % rhs == 0
}

// Функция, которая «не возвращает» значение, на самом деле возвращает единичный тип `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// Когда функция возвращает `()`, возвращаемый тип можно не указывать
fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}
