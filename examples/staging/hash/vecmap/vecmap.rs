use std::collections::VecMap;

struct Tenant<'a> {
    name: &'a str,
    phone: &'a str,
}

fn main() {
    // Start with 5 apartments
    let mut apartments = VecMap::with_capacity(5);

    // The compiler infers 1 as uint
    apartments.insert(1, Tenant {
        name: "John Smith",
        phone: "555-1234",
    });

    apartments.insert(3, Tenant {
        name: "Henrietta George",
        phone: "555-2314",
    });

    apartments.insert(5, Tenant {
        name: "David Rogers",
        phone: "555-5467",
    });

    apartments.remove(&1);
    match apartments.get_mut(&3) {
        Some(henrietta) => henrietta.name = "David and Henrietta Smith",
        _ => println!("Oh no! Where did David and Henrietta go?"),
    }

    apartments.insert(0, Tenant {
        name: "Phillip Davis",
        phone: "5555-7869",
    });

    for (key, tenant) in apartments.iter(){     
        println!("{}: {} ({})", key, tenant.name, tenant.phone);
    }
}
