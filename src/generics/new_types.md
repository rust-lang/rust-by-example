# New Type Idiom

The `newtype` idiom gives compile time guarantees that the right type of value is supplied
to a program.

For example, an age verification function that checks age in years, *must* be given
a value of type `Years`.

```rust, editable
struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}


impl Days {
    /// truncates partial years
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // println!("Old enough {}", old_enough(&age_days));
}
```

Uncomment the last print statement to observe that the type supplied must be `Years`.

### See also:

[`structs`][struct]

[struct]: custom_types/structs.html

