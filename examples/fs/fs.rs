use std::old_io::fs;
use std::old_io::fs::PathExtensions;
use std::old_io::{File, IoResult, USER_RWX};

// A simple implementation of `% cat path`
fn cat(path: &Path) -> IoResult<String> {
    File::open(path).and_then(|mut f| f.read_to_string())
}

// A simple implementation of `% echo s > path`
fn echo(s: &str, path: &Path) -> IoResult<()> {
    File::create(path).and_then(|mut f| f.write_str(s))
}

// A simple implementation of `% touch path` (ignores existing files)
fn touch(path: &Path) -> IoResult<()> {
    if !path.exists() {
        File::create(path).and_then(|_| Ok(()))
    } else {
        Ok(())
    }
}

fn main() {
    println!("`mkdir a`");
    // Create a directory, returns `IoResult<()>`
    match fs::mkdir(&Path::new("a"), USER_RWX) {
        Err(why) => println!("! {:?}", why.kind),
        Ok(_) => {},
    }

    println!("`echo hello > a/b.txt`");
    // The previous match can be simplified using the `unwrap_or_else` method
    echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind);
    });

    println!("`mkdir -p a/c/d`");
    // Recursively create a directory, returns `IoResult<()>`
    fs::mkdir_recursive(&Path::new("a/c/d"), USER_RWX).unwrap_or_else(|why| {
        println!("! {:?}", why.kind);
    });

    println!("`touch a/c/e.txt`");
    touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind);
    });

    println!("`ln -s ../b.txt a/c/b.txt`");
    // Create a symbolic link, returns `IoResult<()>`
    fs::symlink(&Path::new("../b.txt"),
                &Path::new("a/c/b.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind);
    });

    println!("`cat a/c/b.txt`");
    match cat(&Path::new("a/c/b.txt")) {
        Err(why) => println!("! {:?}", why.kind),
        Ok(s) => println!("> {}", s),
    }

    println!("`ls a`");
    // Read the contents of a directory, returns `IoResult<Vec<Path>>`
    match fs::readdir(&Path::new("a")) {
        Err(why) => println!("! {:?}", why.kind),
        Ok(paths) => for path in paths.iter() {
            println!("> {}", path.display());
        },
    }

    println!("`walk a`");
    // Recursively walk over the contents of a directory, returns
    // `Directories`, which implements the `Iterator<Path> trait
    match fs::walk_dir(&Path::new("a")) {
        Err(why) => println!("! {:?}", why.kind),
        Ok(mut paths) => for path in paths {
            println!("> {}", path.display());
        },
    }

    println!("`rm a/c/e.txt`");
    // Remove a file, returns `IoResult<()>`
    fs::unlink(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind);
    });

    println!("`rmdir a/c/d`");
    // Remove an empty directory, returns `IoResult<()>`
    fs::rmdir(&Path::new("a/c/d")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind);
    });
}
