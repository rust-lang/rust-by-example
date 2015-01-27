// FIXME To see the "real" compiler error, change both `&'b` and `&'e` into `&`

fn main() { // `'main` starts ────────────────────────────────────────────┐
    let stack_integer: i32 = 5; // `'a` starts ─────────────────────────┐ │
    let boxed_integer = Box::new(4); // `'b` starts ──────────────────┐ │ │
    //                                                                │ │ │
    // This is a valid operation                                      │ │ │
    let ref_to_box: &'b i32 = &*boxed_integer; // `'c` starts ──────┐ │ │ │
    //                                                              │ │ │ │
    // The compiler forbids this operation, because                 │ │ │ │
    // `ref_to_another_box` would become a dangling pointer         │ │ │ │
    let ref_to_another_box: &'e i32 = { // `'let` `'d` start ───┬─┐ │ │ │ │
        let another_boxed_integer = Box::new(3); // ──────────┐ │ │ │ │ │ │
        // ^ `e` starts                                       │ │ │ │ │ │ │
        &*another_boxed_integer //                            │ │ │ │ │ │ │
    }; // `'e` `'let` end ────────────────────────────────────┴─┘ │ │ │ │ │
    //                                                            │ │ │ │ │
    let invalid_dereference = *ref_to_another_box; //             │ │ │ │ │
} // `'d` `'c` `'b` `'a` `'main` end ─────────────────────────────┴─┴─┴─┴─┘
