// Модуль по имени `my_mod`
mod my_mod {
    // Все элементы модуля по умолчанию являются приватными.
    fn private_function() {
        println!("вызвана `my_mod::private_function()`");
    }

    // Используем модификатор `pub`, чтобы сделать элемент публичным.
    pub fn function() {
        println!("вызвана `my_mod::function()`");
    }

    // Приватные элементы модуля доступны другим элементам
    // данного модуля.
    pub fn indirect_access() {
        print!("вызвана `my_mod::indirect_access()`, которая\n> ");
        private_function();
    }

    // Модули так же могут быть вложенными
    pub mod nested {
        pub fn function() {
            println!("вызвана `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("вызвана `my_mod::nested::private_function()`");
        }

        // Функции объявленные с использованием синтаксиса `pub(in path)` будет видна
        // только в пределах заданного пути.
        // `path` должен быть родительским или наследуемым модулем
        pub(in my_mod) fn public_function_in_my_mod() {
            print!("вызвана `my_mod::nested::public_function_in_my_mod()`, которая\n > ");
            public_function_in_nested()
        }

        // Функции объявленные с использованием синтаксиса `pub(self)` будет видна
        // только в текущем модуле
        pub(self) fn public_function_in_nested() {
            println!("вызвана `my_mod::nested::public_function_in_nested");
        }

        // Функции объявленные с использованием синтаксиса `pub(super)` будет видна
        // только в родительском модуле
        pub(super) fn public_function_in_super_mod() {
            println!("вызвана my_mod::nested::public_function_in_super_mod");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("вызвана `my_mod::call_public_funcion_in_my_mod()`, которая\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) сделает функцию видимой для всего текущего контейнера
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()");
    }

    // Вложенные модули подчиняются тем же правилам видимости
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("вызвана `my_mod::private_nested::function()`");
        }
    }
}

fn function() {
    println!("вызвана `function()`");
}

fn main() {
    // Модули позволяют устранить противоречия между элементами, которые имеют одинаковые названия.
    function();
    my_mod::function();

    // Публичные элементы, включая те, что находятся во вложенном модуле,
    // доступны извне родительского модуля
    my_mod::indirect_access();
    my_mod::nested::function();

    // Приватные элементы модуля не доступны напрямую,
    // даже если вложенный модуль является публичным:

    // Ошибка! функция `private_function` приватная
    //my_mod::private_function();
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку

    // Ошибка! функция `private_function` приватная
    //my_modmy::nested::private_function();
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку

    // Ошибка! Модуль `private_nested` является приватным
    //my_mod::private_nested::function();
    // ЗАДАНИЕ ^ Попробуйте раскомментировать эту строку
}
