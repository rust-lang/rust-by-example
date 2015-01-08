use std::collections::HashSet;

fn main() {
    let mut a: HashSet<int> = vec!(1i, 2, 3).into_iter().collect();
    let mut b: HashSet<int> = vec!(2i, 3, 4).into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // `HashSet::insert()` returns false if
    // there was a value already present.
    assert!(b.insert(4), "Value 4 is already in set B!");
    // FIXME ^ Comment out this line

    b.insert(5);

    // If a collection's element type implements `Show`,
    // then the collection implements `Show`.
    // It usually prints its elements in the format `[elem1, elem2, ...]`
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // Print [1, 2, 3, 4, 5] in arbitrary order
    println!("Union: {}", a.union(&b).collect::<Vec<&int>>());

    // This should print [1]
    println!("Difference: {}", a.difference(&b).collect::<Vec<&int>>());

    // Print [2, 3, 4] in arbitrary order.
    println!("Intersection: {}", a.intersection(&b).collect::<Vec<&int>>());

    // Print [1, 5]
    println!("Symmetric Difference: {}",
             a.symmetric_difference(&b).collect::<Vec<&int>>());
}

