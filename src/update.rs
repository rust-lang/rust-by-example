#![feature(phase)]

extern crate regex;
#[phase(syntax)]
extern crate regex_macros;
extern crate serialize;

use example::Example;

mod example;
mod file;
mod markdown;
mod playpen;

fn main() {
    let examples = Example::get_list();
    let (tx, rx) = channel();

    let mut nexamples = 0;
    for example in examples.move_iter() {
        let tx = tx.clone();
        let count = example.count();

        spawn(proc() {
            example.process(nexamples, tx, 0, String::new());
        });

        nexamples += count;
    }

    let mut entries = range(0, nexamples).map(|_| {
        rx.recv()
    }).collect::<Vec<(uint, String)>>();

    entries.sort_by(|&(i, _), &(j, _)| i.cmp(&j));

    let summary = entries.move_iter()
                         .map(|(_, s)| s)
                         .collect::<Vec<String>>()
                         .connect("\n");

    match file::write(&Path::new("stage/SUMMARY.md"), summary.as_slice()) {
        Err(why) => fail!("{}", why),
        Ok(_) => {},
    }
}
