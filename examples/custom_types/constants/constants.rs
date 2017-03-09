// Константы объявлены в глобальной области видимости.
static LANGUAGE: &'static str = "Rust";
const  THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Получаем доступ к константе внутри функции
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Получаем доступ к константе внутри функции main
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Ошибка! `константы` нельзя изменить.
    THRESHOLD = 5;
    // ИСПРАВЬТЕ ^ Закомментируйте эту строчку
}
