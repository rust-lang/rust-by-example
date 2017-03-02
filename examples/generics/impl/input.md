Подобно функциям, реализации требуют выполнения некоторых условий, чтобы оставаться обобщенными.

```rust
struct S; // Конкретный тип `S`
struct GenericVal<T>(T,); // Обобщенный тип `GenericVal`

// Реализация GenericVal, где мы явно указавыем типы данных параметров:
impl GenericVal<f32> {} // Указываем тип `f32`
impl GenericVal<S> {} // Указываем тип `S`, который мы определили выше

// `<T>` Должен указываться перед типом, чтобы оставаться обобщенным
impl <T> GenericVal<T> {}
```

{impl.play}

### Смотрите также:

[functions returning references][fn], [`impl`][methods], and [`struct`][structs]


[fn]: ../scope/lifetime/fn.html
[methods]: ../fn/methods.html
[specialization_plans]: http://blog.rust-lang.org/2015/05/11/traits.html#the-future
[structs]: ../custom_types/structs.html
