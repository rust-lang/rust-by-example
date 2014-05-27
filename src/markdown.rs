use file::{read,run,write};
use std::io::{UserRWX,fs};

pub struct Markdown<'a> {
    id: &'a str,
    content: String,
}

impl<'a> Markdown<'a> {
    pub fn process(id: &'a str, title: &str) -> Result<Markdown<'a>, String> {
        let mut mkd = try!(Markdown::new(id, title));

        try!(mkd.insert_sources());
        try!(mkd.insert_outputs());
        try!(mkd.save());

        Ok(mkd)
    }

    fn new(id: &'a str, title: &str) -> Result<Markdown<'a>, String> {
        let s = try!(read(&Path::new(format!("examples/{}/input.md", id))));

        Ok(Markdown {
            id: id,
            content: format!("\\# {}\n\n{}", title, s),
        })
    }

    fn insert_sources(&mut self) -> Result<(), String> {
        let re = regex!(r"\{(.*\.rs)\}");

        let mut table = Vec::new();
        for line in self.content.as_slice().lines() {
            match re.captures(line) {
                None => {},
                Some(captures) => {
                    let src = captures.at(1);
                    let input = format!("\\{{}\\}", src);
                    let path = format!("examples/{}/{}", self.id, src);
                    let output = match read(&Path::new(path.as_slice())) {
                        Err(_) => {
                            return Err(format!("{} not found", path));
                        },
                        Ok(string) => {
                            format!("``` rust\n// {}\n{}```",
                                    captures.at(1), string)
                        }
                    };

                    table.push((input, output))
                }
            }
        }

        for (input, output) in table.move_iter() {
            self.content = self.content.replace(input.as_slice(),
                                                output.as_slice());
        }

        Ok(())
    }

    fn insert_outputs(&mut self) -> Result<(), String> {
        let r = regex!(r"\{(.*)\.out\}");

        let dir = Path::new(format!("stage/{}", self.id));

        match fs::mkdir(&dir, UserRWX) {
            Err(_) => {},
            Ok(_) => {},
        }

        let mut table = Vec::new();
        for line in self.content.as_slice().lines() {
            match r.captures(line) {
                None => {},
                Some(captures) => {
                    let src = captures.at(1);
                    let input = format!("\\{{}.out\\}", src);
                    let s = try!(run(self.id, src));
                    let s = format!("```\n$ rustc {0}.rs && ./{0}\n{1}```",
                                    src, s);

                    table.push((input, s));
                },
            }
        }

        for (input, output) in table.move_iter() {
            self.content = self.content.replace(input.as_slice(),
                                                output.as_slice());
        }

        match fs::rmdir(&dir) {
            Err(_) => Err(format!("couldn't remove {}", dir.display())),
            Ok(_) => Ok(()),
        }
    }

    fn save(&self) -> Result<(), String> {
        let path = Path::new(format!("stage/{}.md", self.id));

        write(&path, self.content.as_slice())
    }
}
