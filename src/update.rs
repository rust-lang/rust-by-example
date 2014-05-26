extern crate serialize;

use serialize::{Decodable,json};
use std::io::fs::File;
use std::io::process::{Command,ProcessOutput};
use std::io::{fs,IoResult,Truncate,UserRWX,Write};
use std::str;

#[deriving(Decodable,Show)]
struct Example {
    id: String,
    title: String,
}

fn read(file: &Path) -> IoResult<String> {
    let mut file = try!(File::open(file));

    Ok(try!(file.read_to_str()))
}

fn write(file: &Path, contents: &str) -> IoResult<()> {
    let mut file = try!(File::open_mode(file, Truncate, Write));

    Ok(try!(file.write_str(contents)))
}

fn update(example: &Example) -> bool {
    print!("{}... ", example.id);

    let src_dir = Path::new(format!("src/{}", example.id));
    let out_dir = Path::new(format!("output/examples/{}", example.id));

    let sources: Vec<Path> = match fs::walk_dir(&src_dir) {
        Err(_) => {
            println!("couldn't read {}", src_dir.display());
            return false
        } Ok(contents) => contents.filter(|path| match path.extension_str() {
            None => false,
            Some(extension) => extension == "rs",
        }).collect(),
    };

    let mut template = match read(&src_dir.join("input.md")) {
        Err(_) => {
            println!("couldn't read input.md");
            return false
        } Ok(contents) => format!("\\# {}\n\n{}", example.title, contents),
    };

    // insert source code in markdown
    for source in sources.iter() {
        let path = source.as_str().unwrap().split('/').skip(2).collect::<Vec<&str>>().connect("/");
        let code = match read(source) {
            Err(_) => {
                println!("couldn't read {}", path);
                return false;
            }, Ok(contents) => format!("``` rust\n// {}\n{}```", path, contents),
        };

        template = template.replace(format!("\\{{}\\}", path).as_slice(),
                                    code.as_slice());
    }

    // insert program output in markdown
    for source in sources.iter() {
        let filename = source.filename_str().unwrap();
        let token = format!("\\{{}\\}", source.with_extension("out").filename_str().unwrap());

        if template.as_slice().contains(token.as_slice()) {
            match compile_run(source) {
                Err(action) => {
                    println!("couldn't {} {}", action, filename);
                    return false;
                }, Ok(output) => {
                    let output = format!("```\n$ rustc {} && ./{}\n{}```",
                                         filename,
                                         filename.split('.').nth(0).unwrap(),
                                         output);

                    template = template.replace(token.as_slice(),
                                                output.as_slice());
                }
            }
        }
    }

    if !out_dir.exists() {
        fs::mkdir(&out_dir, UserRWX).unwrap();
    }

    match write(&out_dir.join("README.md"), template.as_slice()) {
        Err(_) => {
            println!("couldn't write README.md");
            false
        } Ok(_) => {
            println!("DONE");
            true
        }
    }
}

fn compile_run(path: &Path) -> Result<String, &'static str> {
    match Command::new("rustc").args([path.as_str().unwrap(), "-o", "executable"]).output() {
        Err(_) => return Err("compile"),
        Ok(out) => {
            if !out.status.success() {
                return Ok(str::from_utf8_owned(out.error).unwrap());
            }
        }
    }

    match Command::new("./executable").output() {
        Err(_) => Err("run"),
        Ok(out) => {
            fs::unlink(&Path::new("./executable")).unwrap();
            let ProcessOutput { status: _, output: out, error: err } = out;
            let stdout = str::from_utf8_owned(out).unwrap();
            let stderr = str::from_utf8_owned(err).unwrap();

            Ok(vec!(stdout, stderr).concat())
        }
    }
}

fn main() {
    let src_dir = Path::new("src");

    let examples: Vec<Example> = match read(&src_dir.join("order.json")) {
        Err(err) => fail!("couldn't read order.json: {}", err),
        Ok(string) => match json::from_str(string.as_slice()) {
            Err(err) => fail!("order.json is invalid json: {}", err),
            Ok(json) => match Decodable::decode(&mut json::Decoder::new(json)) {
                Err(err) => fail!("error decoding order.json: {}", err),
                Ok(data) => data,
            }
        }
    };

    let mut indent = false;
    let summary = examples.move_iter().map(|example| {
        if example.id.as_slice() == "staging" {
            indent = true;
        }

        let chapter = if update(&example) {
            format!("* [{}](examples/{}/README.md)", example.title, example.id)
        } else {
            format!("* {}", example.title)
        };

        if indent &&
            example.id.as_slice() != "staging" &&
            example.id.as_slice() != "todo" {
            format!("  {}", chapter)
        } else {
            chapter
        }
    }).collect::<Vec<String>>().connect("\n");

    if write(&Path::new("output/SUMMARY.md"), summary.as_slice()).is_err() {
        fail!("failed to write SUMMARY.md");
    }
}
