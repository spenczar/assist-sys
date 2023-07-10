# assist-sys

This is a Rust crate which provides bindings for
[ASSIST](https://github.com/matthewholman/assist).

The generated bindings are currently only for a statically linked
version of assist. This isn't for a deep reason; it's just that I
expect that dynamically linking to assist is pretty unusual, and it's
easier to provide a statically linked sys crate.

