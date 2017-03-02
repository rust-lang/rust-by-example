// Модуль по имени `my`
mod my {
    // Все элементы модуля по умолчанию являются приватными.
    fn private_function() {
        println!("вызвана `my::private_function()`");
    }

    // Используем модификатор `pub`, чтобы сделать элемент публичным.
    pub fn function() {
        println!("вызвана `my::function()`");
    }

    // Приватные элементы модуля доступны другим элементам
    // данного модуля.
    pub fn indirect_access() {
        print!("вызвана `my::indirect_access()`, that\n> ");
        private_function();
    }

    // Модули так же могут быть вложенными
    pub mod nested {
        pub fn function() {
            println!("вызвана `my::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("вызвана `my::nested::private_function()`");
        }
    }

    // Вложенные модули подчиняются тем же правилам видимости
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("вызвана `my::private_nested::function()`");
        }
    }
}

fn function() {
    println!("вызвана `function()`");
}

fn main() {
    // Модули позволяют устранить противоречия между элементами, которые имеют одинаковые названия.
    function();
    my::function();

    // Публичные элементы, включая те, что находятся во вложенном модуле,
    // доступны извне родительского модуля
    my::indirect_access();
    my::nested::function();

    // Приватные элементы модуля не доступны напрямую,
    // даже если вложенный модуль является публичным:

    // Ошибка! функция `private_function` приватная
    //my::private_function();
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку

    // Ошибка! функция `private_function` приватная
    //my::nested::private_function();
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку

    // Ошибка! Модуль `private_nested` является приватным
    //my::private_nested::function();
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку
}
