`serialize::Encodable` is a trait implemented for types to make them encodable
by the `serialize` module.

`Encodable` types can be serialized into JSON using `json::encode`.

Just like `Decodable`, `Encodable` can be automatically derived for a struct
using `#[deriving(Encodable)]`.

{encodable.play}
