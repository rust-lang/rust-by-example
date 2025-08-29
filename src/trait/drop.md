# Drop

The [`Drop`][Drop] trait only has one method: `drop`, which is called automatically
when an object goes out of scope. The main use of the `Drop` trait is to free the
resources that the implementor instance owns.

`Box`, `Vec`, `String`, `File`, and `Process` are some examples of types that
implement the `Drop` trait to free resources. The `Drop` trait can also be
manually implemented for any custom data type.

The following example adds a print to console to the `drop` function to announce
when it is called.

```rust,editable
struct Droppable {
    name: &'static str,
}

// This trivial implementation of `drop` adds a print to console.
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // Variable can be manually dropped using the `drop` function
    drop(_a);
    // TODO ^ Try commenting this line

    println!("end of the main function");

    // `_a` *won't* be `drop`ed again here, because it already has been
    // (manually) `drop`ed
}
```

For a more practical example, here's how the `Drop` trait can be used to automatically
clean up temporary files when they're no longer needed:

```rust,editable
use std::fs::File;
use std::path::PathBuf;

struct TempFile {
    file: File,
    path: PathBuf,
}

impl TempFile {
    fn new(path: PathBuf) -> std::io::Result<Self> {
        // Note: File::create() will overwrite existing files
        let file = File::create(&path)?;

        Ok(Self { file, path })
    }
}

// When TempFile is dropped:
// 1. First, our drop implementation will remove the file's name from the filesystem.
// 2. Then, File's drop will close the file, removing its underlying content from the disk.
impl Drop for TempFile {
    fn drop(&mut self) {
        if let Err(e) = std::fs::remove_file(&self.path) {
            eprintln!("Failed to remove temporary file: {}", e);
        }
        println!("> Dropped temporary file: {:?}", self.path);
        // File's drop is implicitly called here because it is a field of this struct.
    }
}

fn main() -> std::io::Result<()> {
    // Create a new scope to demonstrate drop behavior
    {
        let temp = TempFile::new("test.txt".into())?;
        println!("Temporary file created");
        // File will be automatically cleaned up when temp goes out of scope
    }
    println!("End of scope - file should be cleaned up");

    // We can also manually drop if needed
    let temp2 = TempFile::new("another_test.txt".into())?;
    drop(temp2); // Explicitly drop the file
    println!("Manually dropped file");

    Ok(())
}
```

[Drop]: https://doc.rust-lang.org/std/ops/trait.Drop.html
