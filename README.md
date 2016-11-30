# highlight-rs

 [highlight-js](https://github.com/isagalaev/highlight.js) implemented entirely in Rust using emscripten.

 Built on top of the [rust-webplatform](http://github.com/tcr/rust-webplatform) library.

## Compilation

Compiling highlight-rs for the browser requires a nightly Rust.

```
rustup install nightly
rustup override set nightly
rustup target add asmjs-unknown-emscripten
rustup target add wasm32-unknown-emscripten
```

You should also set up emscripten:

```
curl -O https://s3.amazonaws.com/mozilla-games/emscripten/releases/emsdk-portable.tar.gz
tar -xzf emsdk-portable.tar.gz
source emsdk_portable/emsdk_env.sh
emsdk update
emsdk install sdk-incoming-64bit
emsdk activate sdk-incoming-64bit
```

Then you're ready to build:

```
# .cargo/config sets this to build with asmjs-unknown-emscripten as
# a target

cargo build
cp target/asmjs-unknown-emscripten/debug/highlight.js static
cd static
python -m SimpleHTTPServer
```

Open `http://localhost:8000/`. There you go!

See [brson's post on Rust and emscripten](https://users.rust-lang.org/t/compiling-to-the-web-with-rust-and-emscripten/7627) for more installation details.

## License
Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Thanks
Big thanks to the
[rust-todomvc](https://github.com/tcr/rust-todomvc/blob/master) being
a great example to getting stuff working.
