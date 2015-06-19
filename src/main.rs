#![feature(iter_arith)]
#![feature(scoped)]

#![deny(warnings)]
#![allow(deprecated)] // thread::scoped needs to go, but can't do it now
#![feature(plugin)]

extern crate regex;
extern crate rustc_serialize;

use example::Example;
use std::thread;
use std::sync::mpsc;
use std::path::Path;

mod example;
mod file;
mod markdown;
mod playpen;

fn main() {
    let examples = Example::get_list();
    let (tx, rx) = mpsc::channel();

    let mut nexamples = 0;
    for (i, example) in examples.into_iter().enumerate() {
        let tx = tx.clone();
        let count = example.count();

        let _ = thread::scoped(move || {
            example.process(vec!(i + 1), tx, 0, String::new());
        });

        nexamples += count;
    }

    let mut entries = (0..nexamples).map(|_| {
        rx.recv().unwrap()
    }).collect::<Vec<(Vec<usize>, String)>>();

    entries.sort_by(|&(ref i, _), &(ref j, _)| i.cmp(j));

    let summary = entries.into_iter()
                         .map(|(_, s)| s)
                         .collect::<Vec<String>>()
                         .connect("\n");

    match file::write(&Path::new("stage/SUMMARY.md"), &summary) {
        Err(why) => panic!("{}", why),
        Ok(_) => {},
    }
}
