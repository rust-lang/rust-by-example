#![allow(deprecated)]

use std::env;
use std::process::Command;
use std::io::prelude::*;
use std::io;
use std::fs;
use std::fs::File;

pub fn mkdir(path: &Path) {
    match fs::create_dir_all(path) {
        Err(_) => {},
        Ok(_) => {},
    }
}

pub fn run(prefix: &str, id: &str, src: &str) -> Result<String, String> {
    let cwd = env::current_dir().unwrap();
    // Assume that the current working directory actually exists
    let out_dir = cwd.join(&format!("bin/{}/{}", prefix, id));

    match Command::new("rustc")
        .current_dir(&Path::new(format!("examples/{}/{}", prefix, id)))
        .arg(&format!("{}.rs", src))
        .arg("--out-dir")
        .arg(&out_dir)
        .output() {
            Ok(o) => o,
            Err(e) => { return Ok(e.to_string()) },
    };

    let executable = Path::new(format!("./bin/{}/{}/{}", prefix, id, src));

    let output = match Command::new(&executable).output() {
        Ok(o) => o,
        Err(e) => { return Ok(e.to_string()) },
    };

    Ok(String::from_utf8(output.stdout).unwrap())
}

pub fn write(path: &Path, string: &str) -> io::Result<()> {
    let mut f = try!(File::create(path));
    f.write_all(string.as_bytes())
}
