# Opening an issue

## I would like to see a new chapter about $TOPIC

Please check if such topic is already part of our
[TODO list](https://github.com/japaric/rust-by-example/issues/1). If that's
not the case, leave the request as a comment on that issue.

## There's a problem with chapter X

Please include the chapter id in the issue title, e.g. "lifetime: concept not
clear". You can find all the chapters ids
[here](https://github.com/japaric/rust-by-example/blob/master/src/order.json).

# Sending a PR

## Commit message

* If you are submitting a correction/modification to an existing chapter,
  please start the commit message with the chapter id, e.g. "hello: fix typo",
  you can find all the chapters ids
  [here](https://github.com/japaric/rust-by-example/blob/master/src/order.json).

* If you are submitting a new chapter, don't forget to register the chapter in
  the `src/order.json` file, place the entry in the staging area section. Also,
  include the chapter name in the commit message, e.g. "add chapter on Unsafe
  operations".

## Code Style

### Markdown (.md)

* Lines should contain a maximum of 79 characters.
  * This can be waived in the case of long URLs.

### Rust code (.rs)

* Lines should contain a maximum of 79 characters.
