I type using a Colemak keyboard layout. One day I accidentally typed the word
"type" on a Qwerty keyboard and the word "fork" appeared. I wondered how many
such words exist. This little program answers that question.

It reads a list of words from stdin and prints out any that turn into other
words when typed on Qwerty as Colemak. I call them Colemak-Qwerty words. Try
feeding in a [Unix words file](https://en.wikipedia.org/wiki/Words_\(Unix\)).
Note that a couple of the Colemak-Qwerty words are a tad naughty.

This is written using Rust 2018 features, so you'll need Rust 1.31 or later to
build.

Some examples:
* type → fork
* fit → elf
* purer → risks
* talk → faun
