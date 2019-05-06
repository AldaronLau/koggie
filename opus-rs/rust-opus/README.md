# rust-opus

Rust bindings to [libopus](https://www.opus-codec.org/) and
[libopusfile](https://www.opus-codec.org/docs/opusfile_api-0.6/index.html).

If the libraries are installed then only bindings are built. Otherwise the build scripts
try to build the C libraries included in this repository which may not be up to date.

On Windows, msys2 is required for building.
