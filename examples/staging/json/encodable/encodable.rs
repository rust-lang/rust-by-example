extern crate serialize;

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
        // `str_encode` encodes an `Encodable` implementor into a `String`
        println!("{}", json::Encoder::str_encode(city));
    }
}
