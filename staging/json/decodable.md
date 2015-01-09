### 49.10.2 `Decodable`

`serialize::Decodable` is a trait implemented for types to make them decodable
by the `serialize` module.

To deserialize a type from a `Json` enum, a `serialize::json::Decoder` is first
created for this `Json`. It can then be used to decode data into `Decodable`s.

To make this task easier, `Decodable` can be automatically derived for a struct
using `#[deriving(Decodable)]`. This creates a default implementation
in which key-value pairs of a JSON object are mapped to
fields of a `struct`. The keys are expected to have exactly the same
names as the `struct` fields, including case.

<div id="active-code">
<button class="btn btn-primary" type="button" id="run-code">Run</button>
<button class="btn btn-primary" type="button" id="reset-code">Reset</button>
<div id="editor">extern crate serialize;

use serialize::{json, Decodable};

#[deriving(Decodable)]
struct City {
    name: String,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

fn main() {
    for &json&#95;str in [
        r#"{"name": "Cape Town", "lat": -33.925, "lon": 18.424}"#,

        // Any extra fields will be ignored:
        r#"{"name": "Tokyo", "country": "Japan", "lat": -33.925,
            "lon": 18.424}"#,

        // However, a missing field will cause an error:
        r#"{"lat": -33.86, "lon": 151.209}"#,
        // FIXME ^ fill in the missing field: "name": "Sydney"
    ].iter() {
        let json&#95;object = match json::from&#95;str(json&#95;str) {
            Ok(json) =&gt; json,
            Err(err) =&gt; {
                println!("json::from&#95;str: {}", err);
                continue;
            },
        };

        // A &#96;Decoder&#96; object is created based on the &#96;json&#95;object&#96;
        // using which a &#96;City&#96; struct can be decoded:
        let mut decoder = json::Decoder::new(json&#95;object);
        let city: City = match Decodable::decode(&mut decoder) {
            Ok(city) =&gt; city,
            Err(err) =&gt; {
                println!("Decodable::decode: {}", err);
                continue;
            },
        };

        println!("City: {} ({:.2}° N, {:.2}° E)",
                 city.name, city.lat, city.lon);
    }
}</div>
<div id="result"></div>
</div>
