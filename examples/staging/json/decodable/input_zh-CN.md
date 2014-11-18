`serialize::Decodable` is a trait implemented for types to make them decodable
by the `serialize` module.

To deserialize a type from a `Json` enum, a `serialize::json::Decoder` is first
created for this `Json`. It can then be used to decode data into `Decodable`s.

To make this task easier, `Decodable` can be automatically derived for a struct
using `#[deriving(Decodable)]`. This creates a default implementation
in which key-value pairs of a JSON object are mapped to
fields of a `struct`. The keys are expected to have exactly the same
names as the `struct` fields, including case.

{decodable.play}
