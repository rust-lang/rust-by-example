The
[`std::io::fs`](http://static.rust-lang.org/doc/master/std/io/fs/index.html)
module contains all the functions you would expect to deal with the filesystem.

{fs.rs}

```
$ rustc fs.rs && ./fs
`mkdir a`
`echo hello > a/b.txt`
`mkdir -p a/c/d`
`touch a/c/e.txt`
`ln -s ../b.txt a/c/b.txt`
`cat a/c/b.txt`
> hello
`ls a`
> a/b.txt
> a/c
`walk a`
> a/c
> a/c/b.txt
> a/c/e.txt
> a/c/d
> a/b.txt
`rm a/c/e.txt`
`rmdir a/c/d`
```

The final state of the `a` directory is:

```
$ tree a
a
|-- b.txt
`-- c
    `-- b.txt -> ../b.txt

1 directory, 2 files
```
