use file;
use playpen;

pub struct Markdown<'a, 'b> {
    content: String,
    id: &'a str,
    prefix: &'b str,
}

impl<'a, 'b> Markdown<'a, 'b> {
    pub fn process(number: &[uint], id: &'a str, title: &str, prefix: &'b str)
        -> Result<(), String>
    {
        let mut mkd = try!(Markdown::new(number, id, title, prefix));

        try!(mkd.insert_sources());
        try!(mkd.insert_outputs());
        try!(mkd.insert_playpen_links());
        try!(mkd.save());

        Ok(())
    }

    fn new(number: &[uint], id: &'a str, title: &str, prefix: &'b str)
        -> Result<Markdown<'a, 'b>, String>
    {
        let path = Path::new(format!("examples/{}/{}/input.md", prefix, id));
        let body = try!(file::read(&path));
        let version = number.iter().map(|x| {
            format!("{}", x)
        }).collect::<Vec<String>>().connect(".");

        let content = format!("{} {} {}\n\n{}",
                              "#".repeat(number.len()),
                              version,
                              title,
                              body);

        Ok(Markdown {
            content: content,
            id: id,
            prefix: prefix,
        })
    }

    fn insert_sources(&mut self) -> Result<(), String> {
        let id = self.id;
        let prefix = self.prefix;
        let re = regex!(r"\{(.*\.rs)\}");

        let mut table = Vec::new();
        for line in self.content.as_slice().lines() {
            match re.captures(line) {
                None => {},
                Some(captures) => {
                    let src = captures.at(1);
                    let input = format!("{{{}}}", src);
                    let p = format!("examples/{}/{}/{}", prefix, id, src);
                    let output = match file::read(&Path::new(p.as_slice())) {
                        Err(_) => {
                            return Err(format!("{} not found", p));
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
        let id = self.id;
        let prefix = self.prefix;
        let r = regex!(r"\{(.*)\.out\}");

        let dir = Path::new(format!("bin/{}/{}", prefix, id));

        file::mkdir(&dir);

        let mut table = Vec::new();
        for line in self.content.as_slice().lines() {
            match r.captures(line) {
                None => {},
                Some(captures) => {
                    let src = captures.at(1);
                    let input = format!("{{{}.out}}", src);
                    let s = try!(file::run(prefix, id, src));
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

        Ok(())
    }

    fn insert_playpen_links(&mut self) -> Result<(), String> {
        let id = self.id;
        let prefix = self.prefix;
        let re = regex!(r"\{(.*)\.play\}");

        let mut once_ = false;
        let mut table = Vec::new();
        for line in self.content.as_slice().lines() {
            match re.captures(line) {
                None => {},
                Some(captures) => {
                    if once_ {
                        return Err(format!("more than one editor!"))
                    } else {
                        once_ = true;
                    }

                    let input = format!("{{{}.play}}", captures.at(1));
                    let src = format!("{}.rs", captures.at(1));
                    let p = format!("examples/{}/{}/{}", prefix, id, src);
                    let output = match file::read(&Path::new(p.as_slice())) {
                        Err(_) => {
                            return Err(format!("{} not found", p));
                        },
                        Ok(source) => {
                            playpen::editor(source.as_slice())
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

    fn save(&self) -> Result<(), String> {
        let path = Path::new(format!("stage/{}/{}.md", self.prefix, self.id));

        file::write(&path, self.content.as_slice())
    }
}
