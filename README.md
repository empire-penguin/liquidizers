# `liquidizers`
> Rust liquid-dsp bindings.

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![crates.io](http://meritbadge.herokuapp.com/liquidizers)](https://crates.io/crates/liquidizers)
[![docs](https://docs.rs/liquidizers/badge.svg)](https://docs.rs/liquidizers)

[Documentation](https://docs.rs/crate/liquidizers/)

[Release Notes](https://github.com/empire-penguin/liquidizers/tree/master/NEWS.md)

# About

The `liquidizers` crate provides bindings for the `liquid-dsp` library from the
[Liquid-SDR](https://liquidsdr.org/) project. 

# Compatibility

The aim of this project is to track latest liquid-dsp releases as close as possible. 

# Usage

`liquidizers` is a pretty straight forward port of the C API into Rust:

```rust
use liquidizers::liquid::*;
```

You can find more usage examples in
https://github.com/empire-penguin/liquidizers/tree/master/examples.

# Acknowledgements
* Based on [`rust-zmq`]

[`rust-zmq`]: https://github.com/erickt/rust-zmq