# assist-sys

This is a Rust crate which provides bindings for
[ASSIST](https://github.com/matthewholman/assist).

The generated bindings are currently only for a statically linked
version of assist. This isn't for a deep reason; it's just that I
expect that dynamically linking to assist is pretty unusual, and it's
easier to provide a statically linked sys crate.


## Running tests

Tests require ephemeris data. Get them by running 'make' in the
testdata directory. This will use cURL to download ephemerides
files. They're large - 600MB - but necessary.

Then, 'cargo test' runs tests.
