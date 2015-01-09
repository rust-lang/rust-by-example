### 49.10.3 `Encodable`

`serialize::Encodable` is a trait implemented for types to make them encodable
by the `serialize` module.

`Encodable` types can be serialized into JSON using `json::encode`.

Just like `Decodable`, `Encodable` can be automatically derived for a struct
using `#[deriving(Encodable)]`.

<div id="active-code">
<button class="btn btn-primary" type="button" id="run-code">Run</button>
<button class="btn btn-primary" type="button" id="reset-code">Reset</button>
<div id="editor">extern crate serialize;

use serialize::{json, Encodable};

#[deriving(Encodable)]
struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

fn main() {
    for city in [
        City { name: "São Paulo", lat: -23.55,     lon: -46.633333 },
        City { name: "Lima",      lat: -12.043333, lon: -77.028333 },
        City { name: "Santiago",  lat: -33.45,     lon: -70.666667 },
    ].iter() {
        // &#96;encode&#96; encodes an &#96;Encodable&#96; implementor into a &#96;String&#96;
        println!("{}", json::encode(city));
    }
}</div>
<div id="result"></div>
</div>
