extern crate serialize;

use serialize::{json, Decodable};

// JSON source stored in a string
static json: &'static str = "{
    \"firstName\": \"John\",
    \"lastName\": \"Doe\",
    \"age\": 43,
    \"address\": {
        \"street\": \"Downing Street 10\",
        \"city\": \"London\",
        \"country\": \"Great Britain\"
    },
    \"phoneNumbers\": [
        \"+44 1234567\",
        \"+44 2345678\"
    ]
}";

// A structure that will be de-serialized from JSON
#[deriving(Decodable)]
struct Person
{
    firstName: String,
    lastName: String,
    age: int,
    phoneNumbers: Vec<String>
}

fn main() 
{
    // First we need to create a serialize::json::Json object from the source string
    // Note that from_str() returns a Result enum containing
    // either the Json object or an error
    let json_object = json::from_str(json);

    // We can now create a Decoder object and decode into
    // a newly created Person object
    // Decodable::decode() returns a Result as well
    let mut decoder = json::Decoder::new(json_object.unwrap());
    let person: Person = Decodable::decode(&mut decoder).unwrap();

    println!("Person information:");
    println!("  Name: {} {}", person.firstName, person.lastName);
    println!("  Age: {}", person.age);
    for number in person.phoneNumbers.iter()
    {
        println!("  Phone number: {}", number);
    }
}
