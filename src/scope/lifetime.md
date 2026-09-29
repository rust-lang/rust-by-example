# Lifetimes

A *lifetime* is a construct the compiler (or more specifically, its *borrow
checker*) uses to ensure all borrows are valid. Specifically, a variable's
lifetime begins when it is created and ends when it is destroyed. While
lifetimes and scopes are often referred to together, they are not the same.

Take, for example, the case where we borrow a variable via `&`. The
borrow has a lifetime that is determined by where it is declared. As a result,
the borrow is valid as long as it ends before the lender is destroyed. However,
the scope of the borrow is determined by where the reference is used.

In the following example and in the rest of this section, we will see how
lifetimes relate to scopes, as well as how the two differ.

```rust,editable
// Lifetimes are annotated below with lines denoting the creation
// and destruction of each variable.
// `i` has the longest lifetime because its scope entirely encloses
// both `borrow1` and `borrow2`. The duration of `borrow1` compared
// to `borrow2` is irrelevant since they are disjoint.
fn main() {
    let i = 3; // Lifetime for `i` starts. ────────────────┐
    //                                                     │
    { //                                                   │
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
        //                                                ││
        println!("borrow1: {}", borrow1); //              ││
    } // `borrow1` ends. ─────────────────────────────────┘│
    //                                                     │
    //                                                     │
    { //                                                   │
        let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
        //                                                ││
        println!("borrow2: {}", borrow2); //              ││
    } // `borrow2` ends. ─────────────────────────────────┘│
    //                                                     │
}   // Lifetime ends. ─────────────────────────────────────┘
```

Note that no names or types are assigned to label lifetimes.
This restricts how lifetimes will be able to be used as we will see.

### Exercise: Relate two borrows with one lifetime

Task: Write a `longest` function returning the longer of two string slices.

<details><summary>Hint</summary>

Elided lifetimes give each input its own unrelated lifetime, so name one lifetime explicitly and use it for both inputs and the output.

</details>

<details><summary>Solution</summary>

```rust,editable
// Both inputs and the output share `'a`: the result lives exactly as
// long as the shorter-lived input, which the compiler checks.
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let short = "hi";
    let long = "hello, world";
    assert_eq!(longest(short, long), "hello, world");
    println!("longest: {}", longest(short, long));
}
```

</details>
