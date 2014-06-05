JSON (JavaScript Object Notation) is a popular serialization format.
In Rust, support for JSON decoding and enconding is provided by the `serialize` crate.
It can be used like this:

{json.rs}

{json.play}

Produced output:

```
$ rustc json.rs && ./json
Person information:
  Name: John Doe
  Age: 43
  Phone number: +44 1234567
  Phone number: +44 2345678
```

The program decoded JSON data into corresponding fields of the
`Person` structure. Arrays are decoded into a `Vec`.
Since there is no `address` field in `Person`, this key was
simply ignored. Note that the field names are case-sensitive.
