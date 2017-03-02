Существует три типа структур, которые можно создать с помощью ключевого слова `struct`:

* Кортежная структура, которая, в общем, является именованным кортежем.
* Классическую [C структуру][c_struct]
* Единичную (unit) структуру, которая не имеет полей, но может быть полезна для дженериков.

{structs.play}

### Задание

1. Добавьте функцию `rect_area`, которая рассчитывает площадь прямоугольника. (попробуйте использовать "деструктуризацию" (разбор на части) ).
2. Добавьте функцию `square`, которая принимает в качестве аргументов `Point` и `f32`, а возвращает `Rectangle` with its lower left corner on the point, and a width and height corresponding to the `f32`.

### Смотрите также:

[`attributes`][attributes] и [destructuring][destructuring]

[attributes]: ../attribute.html
[c_struct]: http://en.wikipedia.org/wiki/Struct_(C_programming_language)
[destructuring]: ../flow_control/match/destructuring.html
