# if let Expressions

Syntax
IfLetExpression :
   if let Pattern = Scrutineeexcept lazy boolean operator expression BlockExpression
   (else ( BlockExpression | IfExpression | IfLetExpression ) )?

An if let expression is semantically similar to an if expression but in place of a condition operand it expects the keyword let followed by a pattern, an = and a scrutinee operand. If the value of the scrutinee matches the pattern, the corresponding block will execute. Otherwise, flow proceeds to the following else block if it exists. Like if expressions, if let expressions have a value determined by the block that is evaluated.

```rust,editable
let dish = ("Ham", "Eggs");

// this body will be skipped because the pattern is refuted
if let ("Bacon", b) = dish {
    println!("Bacon is served with {}", b);
} else {
    // This block is evaluated instead.
    println!("No bacon will be served");
}

// this body will execute
if let ("Ham", b) = dish {
    println!("Ham is served with {}", b);
}

if let _ = 5 {
    println!("Irrefutable patterns are always true");
}
```

if and if let expressions can be intermixed:
```rust,editable
let x = Some(3);
let a = if let Some(1) = x {
    1
} else if x == Some(2) {
    2
} else if let Some(y) = x {
    y
} else {
    -1
};
assert_eq!(a, 3);
```
An if let expression is equivalent to a match expression as follows:

```rust,editable
if let PATS = EXPR {
    /* body */
} else {
    /*else */
}
```
is equivalent to

```rust,editable
match EXPR {
    PATS => { /* body */ },
    _ => { /* else */ },    // () if there is no else
}
Multiple patterns may be specified with the | operator. This has the same semantics as with | in match expressions:

```rust,editable
enum E {
    X(u8),
    Y(u8),
    Z(u8),
}
let v = E::Y(12);
if let E::X(n) | E::Y(n) = v {
    assert_eq!(n, 12);
}
```
The expression cannot be a lazy boolean operator expression. Use of a lazy boolean operator is ambiguous with a planned feature change of the language (the implementation of if-let chains - see eRFC 2947). When lazy boolean operator expression is desired, this can be achieved by using parenthesis as below:

```rust,editable
// Before...
if let PAT = EXPR && EXPR { .. }

// After...
if let PAT = ( EXPR && EXPR ) { .. }

// Before...
if let PAT = EXPR || EXPR { .. }

// After...
if let PAT = ( EXPR || EXPR ) { .. }
```


### See also:

[`enum`][enum], [`Option`][option], and the [RFC][if_let_rfc]

[enum]: ../custom_types/enum.md
[if_let_rfc]: https://github.com/rust-lang/rfcs/pull/160
[option]: ../std/option.md
