`Iterator::any` - это функция, которая принимает итератор и возвращает `true`,
если любой элемент удовлетворяет предикату. Иначе возвращает `false`. Ее
объявление:

```rust
pub trait Iterator {
    // Тип, по которому выполняется итерирование
    type Item;

    // `any` принимает `&mut self`, что означает заимствование
    // и изменение, но не поглощение `self`.
    fn any<F>(&mut self, f: F) -> bool where
        // `FnMut` означает, что любая захваченная переменная
        // может быть изменена, но не поглощена. `Self::Item`
        // указывает на захват аргументов замыкания по значению.
        F: FnMut(Self::Item) -> bool {}
}
```

{iter_any.play}

### Смотрите также:

[`std::iter::Iterator::any`][any]

[any]: http://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
