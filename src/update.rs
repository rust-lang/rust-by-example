extern crate serialize;

use std::io::{fs,IoResult,Truncate,UserRWX,Write};
use std::io::fs::File;
use serialize::{Decodable,json};

#[deriving(Decodable,Show)]
struct Example {
    id: ~str,
    title: ~str,
}

fn read(file: &Path) -> IoResult<~str> {
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

    let sources: Vec<Path> = match fs::readdir(&src_dir) {
        Err(_) => {
            println!("couldn't read {}", src_dir.display());
            return false
        } Ok(contents) => contents.move_iter().filter(|path| match path.extension_str() {
            None => false,
            Some(extension) => extension == "rs",
        }).collect(),
    };

    if sources.is_empty() {
        println!("couldn't find source files");
        return false
    }

    let mut template = match read(&src_dir.join("input.md")) {
        Err(_) => {
            println!("couldn't read input.md");
            return false
        } Ok(contents) => format!("\\# {}\n\n{}", example.title, contents),
    };

    for source in sources.iter() {
        let filename = source.filename_str().unwrap();
        let code = match read(source) {
            Err(_) => { println!("couldn't read {}", filename); break },
            Ok(contents) => format!("``` rust\n// {}\n{}```", filename, contents),
        };

        template = template.replace(format!("\\{{}\\}", filename), code);
    }

    if !out_dir.exists() {
        fs::mkdir(&out_dir, UserRWX).unwrap();
    }

    match write(&out_dir.join("README.md"), template) {
        Err(_) => {
            println!("couldn't write README.md");
            false
        } Ok(_) => {
            println!("DONE");
            true
        }
    }
}

fn main() {
    let src_dir = Path::new("src");

    let examples: Vec<Example> = match read(&src_dir.join("order.json")) {
        Err(err) => fail!("couldn't read order.json: {}", err),
        Ok(string) => match json::from_str(string) {
            Err(err) => fail!("order.json is invalid json: {}", err),
            Ok(json) => match Decodable::decode(&mut json::Decoder::new(json)) {
                Err(err) => fail!("error decoding order.json: {}", err),
                Ok(data) => data,
            }
        }
    };

    let summary = examples.move_iter().map(|example| {
        if update(&example) {
            format!("* [{}](examples/{}/README.md)", example.title, example.id)
        } else {
            format!("* {}", example.title)
        }
    }).collect::<Vec<~str>>().connect("\n");

    if write(&Path::new("output/SUMMARY.md"), summary).is_err() {
        fail!("failed to write SUMMARY.md");
    }
}
