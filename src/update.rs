#![feature(phase)]

extern crate regex;
#[phase(syntax)]
extern crate regex_macros;
extern crate serialize;

use example::Example;
use file::write;

mod example;
mod file;
mod markdown;

fn main() {
    let examples = Example::get_list();
    let (tx, rx) = channel();
    let nexamples = examples.len();

    for (i, example) in examples.move_iter().enumerate() {
        let tx = tx.clone();

        spawn(proc() {
            tx.send((i, example.copy_id(), example.process()));
        })
    }

    let mut chapters = range(0, nexamples).map(|_| {
        rx.recv()
    }).collect::<Vec<(uint, String, String)>>();

    chapters.sort_by(|&(i, _, _), &(j, _, _)| i.cmp(&j));

    let mut summary = String::new();
    let mut indent = false;
    for (_, id, chapter) in chapters.move_iter() {
        let id = id.as_slice();

        if indent && id != "todo" && id != "staging" {
            summary.push_str(format!("  {}", chapter).as_slice());
        } else {
            summary.push_str(chapter.as_slice());
        }
        summary.push_char('\n');

        if id == "staging" {
            indent = true;
        }
    }

    match write(&Path::new("stage/SUMMARY.md"), summary.as_slice()) {
        Err(why) => fail!("{}", why),
        Ok(_) => {},
    }
}
