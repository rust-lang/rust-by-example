// this source code is not valid, change `&'b` and `&'e` to `&` to make it
// valid
use std::owned::Box;

fn main() { // 'main starts ──────────────────────────────────────────────┐
    let stack_integer = 5; // 'a starts ────────────────────────────────┐ │
    let boxed_integer = box 4; // 'b starts ──────────────────────────┐ │ │
    //                                                                │ │ │
    // This is a valid operation                                      │ │ │
    let ref_to_box: &'b Box<int> = &boxed_integer; // 'c starts ────┐ │ │ │
    //                                                              │ │ │ │
    // The compiler forbids this operation                          │ │ │ │
    let ref_to_another_box: &'e Box<int> = { // 'let 'd start ──┬─┐ │ │ │ │
        let another_boxed_integer = box 3; // 'e starts ──────┐ │ │ │ │ │ │
        //                                                    │ │ │ │ │ │ │
        &another_boxed_integer //                             │ │ │ │ │ │ │
    }; // 'e 'let end ────────────────────────────────────────┴─┘ │ │ │ │ │
    //                                                            │ │ │ │ │
    let box invalid_dereference = *ref_to_another_box; //         │ │ │ │ │
} // 'd 'c 'b 'a 'main end ───────────────────────────────────────┴─┴─┴─┴─┘
