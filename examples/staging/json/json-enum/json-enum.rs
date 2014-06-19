extern crate serialize;

use serialize::json;

fn main() {
    for &json_str in [
        // Valid JSON
        r#"{"name": "London", "lat": 51.507222, "lon": -0.1275}"#,

        // Not a valid JSON, will produce an error
        r#"{"name": "Prague" "lat": 50.082542, "lon": 14.425992}"#,
        // FIXME            ^ a comma is missing here
    ].iter() {
        // `from_str` decodes JSON from a string and returns a `Result`
        // containing either a `Json` enum or an error
        match json::from_str(json_str) {
            Ok(json) => println!("json::from_str(): {}", json),
            Err(err) => println!("json::from_str(): {}", err),
        };
    }
}
