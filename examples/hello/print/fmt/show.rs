use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // Широта
    lat: f32,
    // Долгота
    lon: f32,
}

impl Display for City {
    // `f` - это буфер, данный метод должен записать в него форматированную строку
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` похож на `format!`, только он запишет форматированную строку
        // в буфер (первый аргумент функции)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    for city in [
        City { name: "Дублин", lat: 53.347778, lon: -6.259722 },
        City { name: "Осло", lat: 59.95, lon: 10.75 },
        City { name: "Ванкувер", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Поменяйте {:?} на {}, когда добавите реализацию
        // типажа fmt::Display
        println!("{:?}", *color)
    }
}
