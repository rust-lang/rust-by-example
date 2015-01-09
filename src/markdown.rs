use file;
use playpen;
use std::iter::repeat;
use regex::Regex;

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

        let len = number.len();
        let content = format!("{} {} {}\n\n{}",
                              repeat("#").take(len).collect::<String>(),
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
        let re = Regex::new(r"\{(.*\.rs)\}").unwrap();

        let mut table = Vec::new();
        for line in self.content.as_slice().lines() {
            match re.captures(line) {
                None => {},
                Some(captures) => {
                    let src = captures.at(1).unwrap();
                    let input = format!("{{{}}}", src);
                    let p = format!("examples/{}/{}/{}", prefix, id, src);
                    let output = match file::read(&Path::new(p.as_slice())) {
                        Err(_) => {
                            return Err(format!("{} not found", p));
                        },
                        Ok(string) => {
                            format!("``` rust\n// {}\n{}```",
                                    src, string)
                        }
                    };

                    table.push((input, output))
                }
            }
        }

        for (input, output) in table.into_iter() {
            self.content = self.content.replace(input.as_slice(),
                                                output.as_slice());
        }

        Ok(())
    }

    fn insert_outputs(&mut self) -> Result<(), String> {
        let id = self.id;
        let prefix = self.prefix;
        let r = Regex::new(r"\{(.*)\.out\}").unwrap();

        let dir = Path::new(format!("bin/{}/{}", prefix, id));

        file::mkdir(&dir);

        let mut table = Vec::new();
        for line in self.content.as_slice().lines() {
            match r.captures(line) {
                None => {},
                Some(captures) => {
                    let src = captures.at(1);
                    let input = format!("{{{:?}.out}}", src);
                    let s = try!(file::run(prefix, id, src.unwrap()));
                    let s = format!("```\n$ rustc {0:?}.rs && ./{0:?}\n{1:?}```",
                                    src, s);

                    table.push((input, s));
                },
            }
        }

        for (input, output) in table.into_iter() {
            self.content = self.content.replace(input.as_slice(),
                                                output.as_slice());
        }

        Ok(())
    }

    fn insert_playpen_links(&mut self) -> Result<(), String> {
        let id = self.id;
        let prefix = self.prefix;
        let re = Regex::new(r"\{(.*)\.play\}").unwrap();

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

                    let srcbase = captures.at(1).unwrap();
                    let input = format!("{{{}.play}}", srcbase);
                    let src = format!("{}.rs", srcbase);
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

        for (input, output) in table.into_iter() {
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
