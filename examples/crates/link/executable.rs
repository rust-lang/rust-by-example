// Ссылка на `библиотеку`. Импортируем элементы, как модуль `rary`
extern crate rary;

fn main() {
    rary::public_function();

    // Ошибка! Функция `private_function` приватная
    //rary::private_function();

    rary::indirect_access();
}
