`serialize::json::Json` is an enum capable of containing all of the
JSON value types. It is Rust's way of representing JSON documents.

`Json` can be decoded from a JSON string using `seralize::json::from_str`:

{json-enum.play}
