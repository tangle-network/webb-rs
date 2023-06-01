<h1 align="center">Webb-rs</h1>

<p align="center">
    <strong>The Webb Core SDK for Rust</strong>
    <br />
    <sub> ⚠️ Beta Software ⚠️ </sub>
</p>

<br />

### Downloading metadata from a Substrate node

Use the [`subxt-cli`](https://lib.rs/crates/subxt-cli) tool to download the metadata for your target runtime from a node.

1. Install:
```bash
cargo install subxt-cli
```

2. To Save the metadata of `protocol-substrate`:
Run the release build of the `protocol-substrate` node, then on another terminal run:

```bash
subxt metadata -f bytes > ./metadata/protocol-substrate-runtime.scale
```

3. To Save the metadata of `dkg`:
Run the release build of the `dkg` node, then on another terminal run:

```bash
subxt metadata -f bytes > ./metadata/dkg-runtime.scale
```

Same goes for any future runtimes.

> Note: This defaults to querying the metadata of a locally running node on the default `http://localhost:9933/`. If querying
a different node then the `metadata` command accepts a `--url` argument.

4. Generating the rust code from the metadata:

```bash
cargo build --features generate-substrate
```

> Tip: See the [build.rs](./build.rs) file to see how everything is being generated.


### Generating EVM Contracts

1. Update any contracts inside the [contracts](./contracts) directory.
2. Run
```bash
cargo build --features generate-contracts
```

> Tip: See the [build.rs](./build.rs) file to see how everything is being generated.


### Using Nix (with flakes)

1. Install [Nix](https://nixos.org/download.html)
2. Enable [flakes](https://nixos.wiki/wiki/Flakes) (if you haven't already)
3. Run `nix develop` to enter a shell with all the dependencies installed

Additionally, if you have [direnv](https://direnv.net/) installed, you can run `direnv allow` to automatically enter the shell whenever you enter the directory.

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
