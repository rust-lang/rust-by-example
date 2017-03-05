fn main() {
    // Переменная счётчик
    let mut n = 1;

    // Цикл while будет работать, пока `n` меньше 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Увеличиваем значение счётчика
        n += 1;
    }
}
