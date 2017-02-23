fn used_function() {}

// `#[allow(dead_code)]` - атрибут, который убирает проверку на неиспользуемый код
#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}
// ИСПРАВЬТЕ ^ Добавьте атрибут `dead_code`, чтобы убрать предупреждение

fn main() {
    used_function();
}
