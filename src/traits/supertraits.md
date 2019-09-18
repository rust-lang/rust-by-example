# Supertraits

Rust doesn't have "inheritance", but you can define a trait as being a superset
of another trait. For example:

```rust,editable
trait Person {
    fn name(&self) -> String;
}

// Student is a supertrait of Person.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a supertrait of both Programmer 
// and Student. Implementing CompSciStudent requires you to impl both subtraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My Git username is {}",
        student.name(),
        student.university(),
        student.git_username()
    )
}

fn main() {}
```

### See also:

[The Rust Programming Language chapter on supertraits][trpl_supertraits]

[trpl_supertraits]: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-supertraits-to-require-one-traits-functionality-within-another-trait
