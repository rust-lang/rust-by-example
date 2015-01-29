use std::old_io::USER_RWX;
use std::old_io::fs;
use std::old_io::process::{Command,ProcessOutput};
use std::old_io::{File,Truncate,Write};
use std::os;

pub fn mkdir(path: &Path) {
    match fs::mkdir_recursive(path, USER_RWX) {
        Err(_) => {},
        Ok(_) => {},
    }
}

pub fn read(path: &Path) -> Result<String, String> {
    match File::open(path) {
        Err(_) => Err(format!("couldn't open {}", path.display())),
        Ok(mut file) => match file.read_to_string() {
            Err(_) => Err(format!("couldn't read {}", path.display())),
            Ok(string) => Ok(string),
        }
    }
}

pub fn run(prefix: &str, id: &str, src: &str) -> Result<String, String> {
    let cwd = os::getcwd().unwrap();
    // Assume that the current working directory actually exists
    let out_dir = cwd.join(format!("bin/{}/{}", prefix, id));

    let mut cmd = Command::new("rustc");
    cmd.cwd(&Path::new(format!("examples/{}/{}", prefix, id)));
    cmd.arg(format!("{}.rs", src));
    cmd.arg("--out-dir");
    cmd.arg(out_dir);

    match cmd.output() {
        Err(_) => return Err(format!("couldn't find rustc")),
        Ok(p) => if !p.status.success() {
            return Ok(String::from_utf8(p.error).unwrap());
        },
    }

    let executable = Path::new(format!("./bin/{}/{}/{}", prefix, id, src));

    match Command::new(&executable).output() {
        Err(_) => Err(format!("couldn't find {}", executable.display())),
        Ok(ProcessOutput { error, output, status }) => {
            let mut s = String::from_utf8(output).unwrap();
            if !status.success() {
                s.push_str(String::from_utf8(error).unwrap().as_slice());
            }

            Ok(s)
        }
    }
}

pub fn write(path: &Path, string: &str) -> Result<(), String> {
    match File::open_mode(path, Truncate, Write) {
        Err(_) => Err(format!("couldn't open {}", path.display())),
        Ok(mut file) => match file.write_str(string) {
            Err(_) => Err(format!("couldn't write {}", path.display())),
            Ok(_) => Ok(()),
        }
    }
}
