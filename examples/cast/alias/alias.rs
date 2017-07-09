// `NanoSecond` это новое имя для `u64`.
type NanoSecond = u64;
type Inch = u64;

// Используйте этот атрибут, чтобы не выводить предупреждение
// о именах не в стиле CamelCase
#[allow(non_camel_case_types)]
type u64_t = u64;
// ЗАДАНИЕ ^ Попробуйте удалить атрибут

fn main() {
    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // Обратите внимание, что псевдонимы *не предоставляют* никакой
    // дополнительной безопасности типов, так как *не являются* новыми типами
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}
