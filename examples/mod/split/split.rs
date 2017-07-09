// Эта декларация найдёт файл с именем `my.rs` или `my/mod.rs` и вставит
// его содержимое внутрь модуля с именем `my` в этой области видимости
mod my;

fn function() {
    println!("вызвана `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}
