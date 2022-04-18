<h1 align="center">Webb-rs</h1>

<p align="center">
    <strong>The Webb Core SDK for Rust</strong>
    <br />
    <sub> ⚠️ Beta Software ⚠️ </sub>
</p>

<br />

### Downloading metadata from a Substrate node

Use the [`subxt-cli`](./cli) tool to download the metadata for your target runtime from a node.

1. Install:
```bash
cargo install subxt-cli
```
2. Save the encoded metadata to a file:
```bash
subxt metadata -f bytes > metadata.scale
```

This defaults to querying the metadata of a locally running node on the default `http://localhost:9933/`. If querying
a different node then the `metadata` command accepts a `--url` argument.

## Safety

This crate uses `#![deny(unsafe_code)]` to ensure everything is implemented in
100% Safe Rust.

## Contributing

Want to join us? take a look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

[good-first-issue]: https://github.com/webb-tools/webb-rs/labels/good%20first%20issue
[help-wanted]: https://github.com/webb-tools/webb-rs/labels/help%20wanted

## License

<sup>
Licensed under <a href="LICENSE">Apache License v2.0</a>.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache license, shall
be licensed as above, without any additional terms or conditions.
</sub>
