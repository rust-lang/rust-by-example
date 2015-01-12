use file;
use markdown::Markdown;
use rustc_serialize::{Decodable,json};
use std::iter::AdditiveIterator;
use std::iter::repeat;
use std::sync::mpsc;

#[derive(RustcDecodable)]
pub struct Example {
    children: Option<Vec<Example>>,
    id: String,
    title: String,
}

impl Example {
    pub fn get_list() -> Vec<Example> {
        match file::read(&Path::new("examples/structure.json")) {
            Err(why) => panic!("{}", why),
            Ok(string) => match json::Json::from_str(string.as_slice()) {
                Err(_) => panic!("structure.json is not valid json"),
                Ok(json) => {
                    match Decodable::decode(&mut json::Decoder::new(json)) {
                        Err(_) => panic!("error decoding structure.json"),
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
                   number: Vec<uint>,
                   tx: mpsc::Sender<(Vec<uint>, String)>,
                   indent: uint,
                   prefix: String)
    {
        let id = self.id.as_slice();
        let prefix = prefix.as_slice();
        let title = self.title.as_slice();

        let entry =
            match Markdown::process(number.as_slice(), id, title, prefix) {
                Ok(_) => {
                    let md = if prefix.chars().all(|c| c.is_whitespace()) {
                        format!("{}.md", id)
                    } else {
                        format!("{}/{}.md", prefix, id)
                    };

                    format!("{}* [{}]({})",
                            repeat("  ").take(indent).collect::<String>(),
                            title,
                            md)
                },
                Err(why) => {
                    print!("{}: {}\n", id, why);
                    format!("{}* {}",
                            repeat("  ").take(indent).collect::<String>(),
                            title)
                },
            };

        let _ = tx.send((number.clone(), entry));

        match self.children {
            None => {},
            Some(ref children) => {
                let path = Path::new(format!("stage/{}/{}", prefix, id));

                file::mkdir(&path);

                for (i, example) in children.iter().enumerate() {
                    let tx = tx.clone();
                    let prefix = if prefix.chars().all(|c| c.is_whitespace()) {
                        format!("{}", id)
                    } else {
                        format!("{}/{}", prefix, id)
                    };

                    let mut number = number.clone();
                    number.push(i + 1);
                    example.process(number,
                                    tx,
                                    indent + 1,
                                    prefix);
                }
            },
        }
    }
}
