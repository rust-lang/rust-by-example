fn main() {
    let mut count = 0u32;

    println!("Давайте считать до бесконечности!");

    // Бесконечный цикл
    loop {
        count += 1;

        if count == 3 {
            println!("три");

            // Пропустить оставшуюся часть цикла
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("Всё, достаточно");

            // Выйти из цикла
            break;
        }
    }
}
