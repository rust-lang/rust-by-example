# Opening an issue

## I would like to see an example about $TOPIC

Please check if such topic is already part of our
[TODO list][issues-all]. If that's
not the case, leave your request as a comment on that issue.

## There's a typo/error in example $X
## Example $X is not clear
## I have an idea for example $X

Please include the example id in the issue title, e.g. "variables/mut: concept
not clear". The example id is the relative path in the URL without the html
extension, e.g. URL: `http://rustbyexample.com/variables/scope.html` -> id:
`variables/scope`

If it's something simple like a typo, you can send a PR directly.

# Sending a PR for a small fix

If you are submitting a correction/modification to an existing chapter, please
start the commit message with the example id, e.g. "type/literals: fix typo".

# I want to contribute an example about $TOPIC

## Check if there is an action plan for that topic

Look for
[issues][issues-open]
that have a C-* label:

* C-new: A new chapter, there probably a lot to do here.

* C-expand: Expand an existing chapter, new examples are needed.

* C-split: The current chapter is too long, we want to split it into smaller
  chunks.

* C-taken: Someone is already working in this issue, but if there is a lot of
  work to do, probably you can still help.

## Let us know what are you working on

If an issue about the topic already exists, leave a comment there to let us
know that you'll help. Otherwise, open a new issue mentioning what topic you
plan to work on.

## Hack away

See the [README][readme] for details about how the static site is generated.

## Finally, send a PR

* Don't forget to register the example in the `examples/structure.json` file.

* Include the example id in the commit message header, e.g. for
  `literals/string` use the message "literals: add example about strings"

* Add a `Close #123` to the commit message, to close the issue that's been used
  to track your work.

# Code Style

## Markdown (.md)

* Lines should contain a maximum of 99 characters.
* Use reference style hyperlinks, for example:

Instead of:

    [Goto my URL](http://www.myurl.com)

Use:

    [Goto my URL][1]

    (Bottom of page)
    [1]: http://www.myurl.com

## Rust code (.rs)

* Lines should contain a maximum of 99 characters.
* In comments, types, methods, macros and variables should be wrapped in
  backticks, e.g. ``` `println!` ```

[issues-all]: https://github.com/rust-lang/rust-by-example/issues/
[issues-open]: https://github.com/rust-lang/rust-by-example/issues?labels=&page=1&state=open
[readme]: README.md
