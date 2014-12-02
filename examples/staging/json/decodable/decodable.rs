extern crate serialize;

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
    for &json_str in [
        r#"{"name": "Cape Town", "lat": -33.925, "lon": 18.424}"#,

        // Any extra fields will be ignored:
        r#"{"name": "Tokyo", "country": "Japan", "lat": -33.925,
            "lon": 18.424}"#,

        // However, a missing field will cause an error:
        r#"{"lat": -33.86, "lon": 151.209}"#,
        // FIXME ^ fill in the missing field: "name": "Sydney"
    ].iter() {
        let json_object = match json::from_str(json_str) {
            Ok(json) => json,
            Err(err) => {
                println!("json::from_str: {}", err);
                continue;
            },
        };

        // A `Decoder` object is created based on the `json_object`
        // using which a `City` struct can be decoded:
        let mut decoder = json::Decoder::new(json_object);
        let city: City = match Decodable::decode(&mut decoder) {
            Ok(city) => city,
            Err(err) => {
                println!("Decodable::decode: {}", err);
                continue;
            },
        };

        println!("City: {} ({:.2}° N, {:.2}° E)",
                 city.name, city.lat, city.lon);
    }
}
