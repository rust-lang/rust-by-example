// Lifetimes are annotated with lines denoting when each variable
// is created and destroyed. `i` has the largest lifetime because it's
// scope entirely encloses both `borrow1` and `borrow2`. The sizedness
// of `borrow1` compared with `borrow2` is irrelevant since they are
// disjoint.
fn main() {
    let i = 3; // Lifetime for `i` starts. ───────┐
    //                                            │
    { //                                          │
        let borrow1 = &i; // `borrow1` starts. ──┐│
        //                                       ││
        println!("borrow1: {}", borrow1); //     ││
    } // `borrow1 ends. ─────────────────────────┘│
    //                                            │
    //                                            │
    { //                                          │
        let borrow2 = &i; // `borrow2` starts. ──┐│
        //                                       ││
        println!("borrow2: {}", borrow2); //     ││
    } // `borrow2` ends. ────────────────────────┘│
    //                                            │
}   // Lifetime ends. ────────────────────────────┘
