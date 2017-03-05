fn main() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box содержит в себе {}", immutable_box);

    // Ошибка изменяемости
    //*immutable_box = 4;

    // *Переместить* упаковку, изменив её владение (и изменяемость)
    let mut mutable_box = immutable_box;

    println!("mutable_box содержит в себе {}", mutable_box);

    // Изменяем данные внутри упаковки
    *mutable_box = 4;

    println!("mutable_box now содержит в себе {}", mutable_box);
}
