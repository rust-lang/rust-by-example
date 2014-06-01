use std::io::fs;
use std::io::process::{Command,ProcessOutput};

struct Executable {
    path: Path,
}

impl Executable {
    // compile source and return the executable created
    fn new(source: &Path) -> Option<Executable> {
        let display = source.display();

        println!("compiling {}...", display);

        match Command::new("rustc").arg(source).output() {
            Err(why) => {
                println!("couldn't spawn rustc: {}", why.desc)

                return None;
            },
            Ok(ProcessOutput { output: _, error: err, status: exit }) => {
                if !exit.success() {
                    print!("compilation failed: \n{}",
                           String::from_utf8(err).unwrap());

                    return None;
                } else {
                    println!("successfully compiled {}", display);
                }
            },
        }

        source.filename_str().and_then(|string| {
            Some(Executable {
                path: Path::new(string.split('.').next().unwrap())
            })
        })
    }

    // run executable
    fn run(&self) {
        let display = self.path.display();

        match Command::new(format!("./{}", display)).output() {
            Err(why) => {
                println!("couldn't execute ./{}: {}", display, why);
            },
            Ok(ProcessOutput { output: out, error: err, status: exit }) => {
                if exit.success() {
                    print!("output was:\n{}", String::from_utf8(out).unwrap())
                } else {
                    let out = String::from_utf8(out).unwrap();
                    let err = String::from_utf8(err).unwrap();

                    print!("runtime error:\n{}{}", out, err);
                }
            },
        }
    }
}

// this Drop implementation will take care of removing the executable file
impl Drop for Executable {
    fn drop(&mut self) {
        let display = self.path.display();

        match fs::unlink(&self.path) {
            Err(_) => println!("couldn't remove {}", display),
            Ok(_) => println!("deleted {}", display),
        }
    }
}

fn main() {
    if true {
        match Executable::new(&Path::new("hello.rs")) {
            None => {},
            Some(executable) => {
                executable.run();

                println!("end of the match block");

                // executable gets "dropped" here
            },
        };

        println!("end of the if block")
    }

    if !Path::new("hello").exists() {
        println!("hello no longer exists")
    }
}
