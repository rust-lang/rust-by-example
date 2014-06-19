use std::fmt;
use std::num::abs;

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl fmt::Show for City {
    // Will output City's name as well as its location in a nice format
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
        let lon_c = if self.lon >= 0.0 {'E'} else {'W'};

        // `write!` writes formatted string into a `io::Writer`
        // and handles `Result` propagation
        write!(f, "{} ({:.3f}° {}, {:.3f}° {})",
               self.name, abs(self.lat), lat_c, abs(self.lon), lon_c)
    }
}

fn main() {
    for city in [
        City{name: "Vancouver", lat: 49.25, lon: -123.1,},
        City{name: "Dublin", lat: 53.347778, lon: -6.259722,},
        City{name: "Oslo", lat: 59.95, lon: 10.75,},
    ].iter() {
        println!("{}", city);
    }
}
