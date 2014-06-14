use file;
use markdown::Markdown;
use serialize::{Decodable,json};
use std::iter::AdditiveIterator;

#[deriving(Decodable)]
pub struct Example {
    children: Option<Vec<Example>>,
    id: String,
    title: String,
}

impl Example {
    pub fn get_list() -> Vec<Example> {
        match file::read(&Path::new("examples/structure.json")) {
            Err(why) => fail!("{}", why),
            Ok(string) => match json::from_str(string.as_slice()) {
                Err(_) => fail!("structure.json is not valid json"),
                Ok(json) => {
                    match Decodable::decode(&mut json::Decoder::new(json)) {
                        Err(_) => fail!("error decoding structure.json"),
                        Ok(examples) => examples,
                    }
                }
            }
        }
    }

    pub fn count(&self) -> uint {
        match self.children {
            None => 1,
            Some(ref children) => 1 + children.iter().map(|c| c.count()).sum(),
        }
    }

    pub fn process(&self,
                   pos: uint,
                   tx: Sender<(uint, String)>,
                   indent: uint,
                   prefix: String)
    {
        let id = self.id.as_slice();
        let prefix = prefix.as_slice();
        let title = self.title.as_slice();

        let entry = match Markdown::process(id, title, prefix) {
            Ok(_) => {
                let md = if prefix.as_slice().is_whitespace() {
                    format!("{}.md", id)
                } else {
                    format!("{}/{}.md", prefix, id)
                };

                format!("{}* [{}]({})",
                        "  ".repeat(indent),
                        title,
                        md)
            },
            Err(why) => {
                print!("{}: {}\n", id, why);
                format!("{}* {}", "  ".repeat(indent), title)
            },
        };

        tx.send((pos, entry));

        match self.children {
            None => {},
            Some(ref children) => {
                let path = Path::new(format!("stage/{}/{}", prefix, id));

                file::mkdir(&path);

                for (i, example) in children.iter().enumerate() {
                    let tx = tx.clone();
                    let prefix = if prefix.as_slice().is_whitespace() {
                        format!("{}", id)
                    } else {
                        format!("{}/{}", prefix, id)
                    };

                    example.process(pos + i + 1,
                                    tx,
                                    indent + 1,
                                    prefix);
                }
            },
        }
    }
}
