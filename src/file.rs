use std::io::fs;
use std::io::process::{Command,ProcessOutput};
use std::io::{File,Truncate,Write};
use std::os;

pub fn read(path: &Path) -> Result<String, String> {
    match File::open(path) {
        Err(_) => Err(format!("couldn't open {}", path.display())),
        Ok(mut file) => match file.read_to_str() {
            Err(_) => Err(format!("couldn't read {}", path.display())),
            Ok(string) => Ok(string),
        }
    }
}

pub fn run(id: &str, filename: &str) -> Result<String, String> {
    let cwd = os::getcwd();
    let out_dir = cwd.join(format!("stage/{}", id));

    let mut cmd = Command::new("rustc");
    cmd.cwd(&Path::new(format!("examples/{}", id)));
    cmd.arg(format!("{}.rs", filename));
    cmd.arg("--out-dir");
    cmd.arg(out_dir);

    match cmd.output() {
        Err(_) => return Err(format!("couldn't find rustc")),
        Ok(p) => if !p.status.success() {
            return Ok(String::from_utf8(p.error).unwrap());
        },
    }

    let executable = Path::new(format!("./stage/{}/{}", id, filename));

    match Command::new(&executable).output() {
        Err(_) => Err(format!("couldn't find {}", id)),
        Ok(ProcessOutput { error: error, output: output, status: status }) => {
            match fs::unlink(&executable) {
                Err(_) => return Err(format!("couldn't remove {}", id)),
                Ok(_) => {},
            }

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
