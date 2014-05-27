use file::read;
use markdown::Markdown;
use serialize::{Decodable,json};

#[deriving(Decodable)]
pub struct Example {
    id: String,
    title: String,
}

impl Example {
    pub fn get_list() -> Vec<Example> {
        match read(&Path::new("examples/order.json")) {
            Err(why) => fail!("{}", why),
            Ok(string) => match json::from_str(string.as_slice()) {
                Err(_) => fail!("order.json is not valid json"),
                Ok(json) => {
                    match Decodable::decode(&mut json::Decoder::new(json)) {
                        Err(_) => fail!("error decoding order.json"),
                        Ok(examples) => examples,
                    }
                }
            }
        }
    }

    pub fn copy_id(&self) -> String {
        self.id.clone()
    }

    pub fn process(&self) -> String {
        match Markdown::process(self.id.as_slice(), self.title.as_slice()) {
            Ok(_) => {
                format!("* [{}]({}.md)", self.title, self.id)
            },
            Err(why) => {
                print!("{}: {}\n", self.id, why);
                format!("* {}", self.title)
            },
        }

    }
}
