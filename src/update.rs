#![deny(warnings)]
#![feature(phase)]

extern crate regex;
#[phase(plugin)]
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
    for (i, example) in examples.into_iter().enumerate() {
        let tx = tx.clone();
        let count = example.count();

        spawn(proc() {
            example.process(vec!(i + 1), tx, 0, String::new());
        });

        nexamples += count;
    }

    let mut entries = range(0, nexamples).map(|_| {
        rx.recv()
    }).collect::<Vec<(Vec<uint>, String)>>();

    entries.sort_by(|&(ref i, _), &(ref j, _)| i.cmp(j));

    let summary = entries.into_iter()
                         .map(|(_, s)| s)
                         .collect::<Vec<String>>()
                         .connect("\n");

    match file::write(&Path::new("stage/SUMMARY.md"), summary.as_slice()) {
        Err(why) => panic!("{}", why),
        Ok(_) => {},
    }
}
