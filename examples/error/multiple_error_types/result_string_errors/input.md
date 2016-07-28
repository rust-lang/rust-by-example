From our previous example, one method of solving our issue with `unwrap` is to remove it.
In doing so, we must move from implicit to explicit error handling. Since the only 
types in play are `Option` and `Result`, we can consider converting both into 
`Result`s with the same `Err` type. For our first attempt at this solution, 
let's try using a `String` for our error:

{result_string.play}

This is not too bad, but it is hardly as nice as the original (it can still be nicer but
we are not there yet). Unfortunately, this approach scales poorly with increasing 
numbers of `Result`s, as will be seen in the next example.
