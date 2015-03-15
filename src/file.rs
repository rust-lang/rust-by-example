#![allow(deprecated)]

use std::env;
use std::process::Command;
use std::io::prelude::*;
use std::io;
use std::fs;
use std::fs::File;
use std::path::Path;

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
    let path_str = &format!("examples/{}/{}", prefix, id);

    match Command::new("rustc")
        .current_dir(&Path::new(path_str))
        .arg(&format!("{}.rs", src))
        .arg("--out-dir")
        .arg(&out_dir)
        .output() {
            Ok(o) => o,
            Err(e) => { return Ok(e.to_string()) },
    };

    let exec_str = &format!("./bin/{}/{}/{}", prefix, id, src);
    let executable = Path::new(exec_str);

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
