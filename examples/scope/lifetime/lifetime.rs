// Lifetimes are annotated with a lines denoting
// when each variable is created and destroyed:
fn main() {
    let i = 3; // Lifetime for `i` starts. ───────┐
    //                                            │
    { //                                          │
        let borrow = &i; // Borrow starts. ──────┐│
        //                                       ││
        println!("Borrowed `i`: {}", borrow); // ││
    } // Borrow ends. ───────────────────────────┘│
    //                                            │
}   // Lifetime ends. ────────────────────────────┘
