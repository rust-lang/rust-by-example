use markdown::Markdown;
use rustc_serialize::{Decodable,json};
use std::iter::repeat;
use std::sync::mpsc;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

#[derive(RustcDecodable)]
pub struct Example {
    children: Option<Vec<Example>>,
    id: String,
    title: String,
}

impl Example {
    pub fn get_list() -> Vec<Example> {

        let mut f = File::open(&Path::new("examples/structure.json")).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();

        match json::Json::from_str(&s) {
            Err(_) => panic!("structure.json is not valid json"),
            Ok(json) => {
                match Decodable::decode(&mut json::Decoder::new(json)) {
                    Err(_) => panic!("error decoding structure.json"),
                    Ok(examples) => examples,
                }
            }
        }
    }

    pub fn count(&self) -> usize {
        match self.children {
            None => 1,
            Some(ref children) => 1 + children.iter()
                                              .map(|c| c.count())
                                              .fold(0, |sum, i| sum + i),
        }
    }

    pub fn process(&self,
                   number: Vec<usize>,
                   tx: mpsc::Sender<(Vec<usize>, String)>,
                   indent: usize,
                   prefix: String)
    {
        let id = &self.id;
        let prefix = &prefix;
        let title = &self.title;

        let entry =
            match Markdown::process(&number, id, title, prefix) {
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
                let path_str = &format!("stage/{}/{}", prefix, id);
                let path = Path::new(path_str);

                fs::create_dir_all(&path).unwrap();

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
