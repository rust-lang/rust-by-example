We've seen that the `Option` enum can be used as a return value from functions
that may fail, where `None` can be returned to indicate failure. However,
sometimes it is important to express *why* an operation failed. To do this we
have the `Result` enum.

The `Result<T, E>` enum has two variants:

* `Ok(value)` which indicates that the operation succeeded, and wraps the
  `value` returned by the operation. (`value` has type `T`)
* `Err(why)`, which indicates that the operation failed, and wraps `why`,
  which (hopefully) explains the cause of the failure. (`why` has type `E`)

{result.play}
