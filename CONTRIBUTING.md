# Rust by Example contribution guidelines

Thank you for your interest in making Rust by Example (also known as RBE)
better! We'd love to have your contribution. We expect all contributors to abide
by the [Rust code of conduct], which you can find at that link or in the
[`CODE_OF_CONDUCT.md`] file in this repository.

[Rust code of conduct]: https://www.rust-lang.org/policies/code-of-conduct
[`CODE_OF_CONDUCT.md`]: https://github.com/rust-lang/rust-by-example/blob/master/CODE_OF_CONDUCT.md

## License

RBE is dual licenced under the MIT and Apache 2.0 licenses, and so are all
contributions. Please see the [`LICENSE-MIT`] and [`LICENSE-APACHE`] files in
this directory for more details.

[`LICENSE-MIT`]: https://github.com/rust-lang/rust-by-example/blob/master/LICENSE-MIT
[`LICENSE-APACHE`]: https://github.com/rust-lang/rust-by-example/blob/master/LICENSE-APACHE

## Pull Requests

To make changes to RBE, please send in pull requests on GitHub to the `master`
branch. We'll review them and either merge or request changes. Travis CI tests
everything as well, so you may get feedback from it too.

If you make additions or other changes to a pull request, feel free to either
amend previous commits or only add new ones, however you prefer. We may ask you
to squash your commits before merging, depending.

## Issue Tracker

You can find the issue tracker
[on GitHub](https://github.com/rust-lang/rust-by-example/issues). If you've
found a problem with RBE, please open an issue there.

We use the following labels:

- `enhancement`: This is for any request for new sections or functionality.
- `bug`: This is for anything that's in RBE, but incorrect or not working.
- `discussion`: A discussion about improving something in RBE; this may lead to
  new enhancement or bug issues.
- `E-mentor`: This issue has someone dedicated to helping a new contributor fix
  it! Can apply to both enhancement or bug issues.

## Development workflow

To build RBE, [install Rust](https://www.rust-lang.org/tools/install), and then:

```bash
$ git clone https://github.com/rust-lang/rust-by-example
$ cd rust-by-example
$ cargo install mdbook
$ mdbook build
```

**The following warnings can be ignored safely.**

```
[WARN] (mdbook::preprocess::cmd): The command wasn't found, is the "gettext" preprocessor installed?
[WARN] (mdbook::preprocess::cmd):   Command: mdbook-gettext
```

[install Rust]: http://rust-lang.org/install.html

The files will be in the `book` directory at the top-level; `mdbook serve` will
open the contents in your web browser ([localhost:3000](http://localhost:3000)
by default).

To run the tests:

```bash
$ mdbook test
```

If you're adding a new chapter, you'll need to edit `src\SUMMARY.md` to add it.
If you're tweaking an existing example, you'll need to edit the corresponding
file; check `src\SUMMARY.md` to see a mapping of where chapters go to files.
