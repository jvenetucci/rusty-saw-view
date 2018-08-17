# rusty-saw-view
Copyright (c) 2018 Joseph Venetucci (venetuc@pdx.edu)
## Overview
rusty-saw-view is a Rust program that helps visualize blockchain data from [Hyperledger Sawtooth](https://www.hyperledger.org/projects/sawtooth). Sawtooth is a blockchain framework that provides a REST API to query information such as the current state of the ledger, individual blocks and transaction, and the entire blockchain itself. Requesting this information from the respective HTTP endpoints returns a JSON formatted block of data.

From my own experience, Sawtooth is great for including a blockchain based distributed ledger into an application. However I learned that explaining how transactions were being made and how the blockchain worked was difficult with people who were unfamiliar with the concept of a blockchain. For transactions and blockchains that were simple I could easily draw it on a whiteboard to help explain it. As the transactions and application got more complex, so did the difficulty in explaining what was going on. Showing the JSON data wasn't really to helpful because (1) the data in each block is serialized and encoded (2) and while it's in human readable JSON format, it's hard to connect each block/transaction to each other when it's just one stream of text.

rusty-saw-view attempts to solve this problem by contacting the HTTP endpoints of a Sawtooth node and requesting the JSON data. It's job is to parse and visually display this data in a way thats easy to understand and explain.

[Rustdocs](https://github.com/jvenetucci/rusty-saw-view#building-the-rustdocs) | [Usage](https://github.com/jvenetucci/rusty-saw-view#usage) | [Usage Guide](https://github.com/jvenetucci/rusty-saw-viewhttps://github.com/jvenetucci/rusty-saw-view#usage-guide)

---

## Table of Contents
- [Features & Requirements](https://github.com/jvenetucci/rusty-saw-view#features--requirements)
- [Project Structure](https://github.com/jvenetucci/rusty-saw-view#project--directory-structure)
- [How to Build](https://github.com/jvenetucci/rusty-saw-view#how-to-build)
  - [Building The Application](https://github.com/jvenetucci/rusty-saw-view#building-the-application)
  - [Teardown & Cleanup](https://github.com/jvenetucci/rusty-saw-view#teardown--cleanup)
  - [Building The Rustdocs](https://github.com/jvenetucci/rusty-saw-view#building-the-rustdocs)
- [Usage](https://github.com/jvenetucci/rusty-saw-view#usage)
  - [CLI Options](https://github.com/jvenetucci/rusty-saw-viewhttps://github.com/jvenetucci/rusty-saw-view#cli-options)
    - [Supported Deserialization Methods](https://github.com/jvenetucci/rusty-saw-view#supported-deserialization-methods)
    - [Custom Deserialization Methods](https://github.com/jvenetucci/rusty-saw-view#adding-unsupported-deserialization-methods)
  - [Usage Guide](https://github.com/jvenetucci/rusty-saw-viewhttps://github.com/jvenetucci/rusty-saw-view#usage-guide)
- [Testing Tools](https://github.com/jvenetucci/rusty-saw-view#testing-tools)
- [Licensing](https://github.com/jvenetucci/rusty-saw-view#licensing)
- [Authors](https://github.com/jvenetucci/rusty-saw-view#authors)
- [Acknowledgments](https://github.com/jvenetucci/rusty-saw-view#acknowledgments)
- [References](https://github.com/jvenetucci/rusty-saw-view#references)

---

## Features & Requirements
- [x] Parse blockchain data stored as JSON files.
- [x] Parse blockchain data by requesting it from the HTTP Endpoints.
- [x] Support multiple deserialization & decoding formats.
    - [X] CBOR.
    - [X] Javascript.
    - [X] BASE64.
    - [X] [Support for advanced and custom protocols like Google Protobuf.](https://github.com/jvenetucci/rusty-saw-view#adding-unsupported-deserialization-methods)
- [X] Display parsed data through the command line in text format.
    - [X] Formatted & Colored output to terminal.
    - [X] Formatted output for piping to file.
- [ ] Display parsed data with a GUI.
    - [ ] Ability to select each block/transaction to learn more about it.

---

## Project & Directory Structure
The root directory of the project contains the following folders:
- **src** -- Contains code for the application.
  - The files in here are further broken into modules. For information on individual modules see
    [Building RustDocs](https://github.com/jvenetucci/rusty-saw-view#building-the-rustdocs)
- **example-blockchain** -- Contains example JSON data for a simple Sawtooth blockchain; See [Usage Guide](https://github.com/jvenetucci/rusty-saw-viewhttps://github.com/jvenetucci/rusty-saw-view#usage-guide).
<!-- - **test** -- Integration test for the application. -->
- **Cargo.toml** -- Contains metadata & the dependices of the application

---

## How to Build
This section will provide an overview of how to build the application. Rustc and [Cargo](https://doc.rust-lang.org/stable/cargo/) are required. Once the application has been built, see
the [Usage Guide](https://github.com/jvenetucci/rusty-saw-viewhttps://github.com/jvenetucci/rusty-saw-view#usage-guide) on how to use it.

### Building The Application
Navigate to the root of the project directory and run the following:
```Bash
$ cargo build
```

### Teardown & Cleanup
To remove the compiled files from building the program run the following in the root of the project directory:
```Bash
$ cargo clean
```

### Building The Rustdocs
Every source file has been annotated with rustdoc comments. Using the [Rustdoc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html) tool will generate nicely formatted HTML documentation for the application. Refer to this documentation if you want to learn more about the individual modules and source code of this application.

To build and view the rustdocs, run the following in the root of the project directory:
```Bash
$ cargo doc --open --no-deps
```
---

## Usage
In this section you'll learn how to use the application by invoking it through the command line. For information on how this is done, see the subsection [CLI Options](https://github.com/jvenetucci/rusty-saw-view#cli-options). If you want to try out the program but don't have access to a running instance of a sawtooth application, see the [Usage Guide](https://github.com/jvenetucci/rusty-saw-view#usage-guide) section. There you'll use provided blockchain data to play with the application.

### CLI Options
After building the project you can run it using `cargo run`. Running the command `cargo run -- ---help` will display instructions on how to use the program. If you want concrete examples on how to invoke it, see the [Usage Guide](https://github.com/jvenetucci/rusty-saw-view#usage-guide).

```bash
USAGE:
  rusty-saw-view [FLAGS] <endpoint> <method> <source> <location>

FLAGS:
  -f, --full-addr    Prints out full addresses & PubKeys
  -g, --genesis      Prints out the settings state or genesis block depending on the context
  -h, --help         Prints help information
  -n, --no-color     Prints without colored text. Use for piping to file
  -V, --version      Prints version information

ARGS:
  <endpoint>    From which endpoint is the data coming from? [possible values: state, blocks]
  <method>      What deserialization method to use? [possible values: cbor, json, custom]
  <source>      Where is the data coming from? [possible values: file, url]
  <location>    File path, or URL to data
```

#### Supported Deserialization Methods
Out of the box the following deserialization methods are supported:
- CBOR
- JSON

### Adding Unsupported Deserialization Methods
Since Sawtooth is modular by design, it allows developers to use whatever serialization scheme they want. This made the development of this application difficult because it can't possibly support every scheme out of the box. For methods not listed in the previous section, users will have to add it to a section of the project in order to use it.

Located in `src/json_deserialize.rs` is a method called `parse_custom()`. Users will need to impliment this method. For more information on this step, see the rustdoc comments that annotate the method. Once this is done you should be able to run the program with `custom` as the method.

### Usage Guide
This section will walk you through using the application by invoking it through the command line. If you want to try out the program but don't have immediate access to JSON blockchain data, then you can use the provided resources in the `/example-blockchain` directory. You'll be able to parse and view blockchain data from the `/state` and `/blocks` endpoint. This section covers pulling data from [files](https://github.com/jvenetucci/rusty-saw-view#data-from-files) and [HTTP Endpoints](https://github.com/jvenetucci/rusty-saw-view#data-from-endpoints).

#### Data From Files
Inside of `/example-blockchain` are two files: `blocks.json` and `state.json` that represent the data from the `/blocks` and `/state` endpoints of a sawtooth node. The particular node was running an instance of the [IntKey](https://sawtooth.hyperledger.org/docs/core/releases/1.0/transaction_family_specifications/integerkey_transaction_family.html) transaction processor. The idea behind it is that it lets you set integer values to variables, and then either increment or decrement them by any integer value. The json files were gathered after five commands had been run. The sequence of commands and current state is explained in `example-blockchain/README.md`.

First lets view the current state of the blockchain, which is viewable from the `/state` endpoint and is stored in the `state.json` file. Run the following:

```bash
cargo run -- state cbor file example-blockchain/state.json
```

Here is the reasoning behind each command after the `--`:
- `state` - We want to decode from the state endpoint.
- `cbor` - Our data is serialized using CBOR. This is blockchain dependent. Other blockchains could use something else.
- `file` - Telling the program that the data is coming from a file.
- `example-blockchain/state.json` - This is the location to the file.

After running that command you should see some output in your terminal window. What should of printed are two pieces of state. Each one has an address and the data stored at that address. From it we can see that two variables are present: num1 with a value of 1, and num2 with a value of 12.

Now lets try exploring each of the blocks in the blockchain. This data is found at the `/blocks` endpoint and is stored in the `blocks.json` file. Run the following command:
```bash
cargo run -- blocks cbor file example-blockchain/blocks.json
```

This time you should see a very different output. What you are seeing is information about each block in the chain. There should be 5 blocks displayed, each with information about them and the action they performed. You can see that in Block 1 that the variable num1 was originally set to 2.

Notice that only the first 6 and last 4 characters of an address, ID, or Public Key is displayed. If you want to see the full string use the `--full-addr` flag:
```bash
cargo run -- blocks cbor file example-blockchain/blocks.json -f
```

What if you wanted to pipe the output to a file? Well you can do that! By default the program prints to the terminal with color (If you're using a Windows terminal you might not see them). You should turn off the coloring when piping to a file with the `no-color` flag. To pipe to a file called `output.txt` with full addresses and no colors run the following:
```bash
cargo run -- blocks cbor file example-blockchain/blocks.json -fn > output.txt
```

If you're familiar with blockchains you may know that each chain starts off with a genesis block, aka Block 0. By default this program does not display it because the data contaiend in it is often serialized differently than the rest of the blockchain. Since this program does not support multiple deserialization methods at run time the data in the block won't be decoded. If you want to print out the block use the `-genesis` flag.
```bash
cargo run -- blocks cbor file example-blockchain/blocks.json -g
```

Running that command on the state data will show the address where the settings data is stored. Any of the flags you've used here for the blocks data will also work on the state data, so have fun!

Now that you've parsed and displayed data from files check out the next section which will show you how to grab data from a sawtooth node via a URL. You'll even get to interact with a live instance of a IntKey processor!

#### Data From Endpoints

---

## Testing Tools
Each source file contains unit tests for the methods defined in it. There are over 50 unit tests for the program. The HTTP requests are tested using [Mockito](https://github.com/lipanski/mockito). Located in the `/test` directory are integration tests. These test can be run with cargo:
```Bash
$ cargo test
```

---

## Licensing
This project is licensed under the MIT License. For a copy of the license see LICENSE.md in the root directory.

The copyright to this project belongs to Joseph Venetucci.

---

## Authors
For any questions related to the project you can contact the author:
- Joseph Venetucci - <venetuc@pdx.edu>

---

## Acknowledgments
This project was made for a college summer class on Programming in Rust that I took in 2018. With the help of the class lectures and professor I was able to gain enough knowledge of rust in order to create this.

[Hyperledger Sawtooth](https://www.hyperledger.org/projects/sawtooth) for if it didn't exist, this project wouldn't either.

I used a few crates created by others as part of this project. The full list of them can be seen in `/Cargo.toml`

---

## References
- [GitHub Repo](https://github.com/jvenetucci/lace)
- [Hyperledger Sawtooth Docs](https://sawtooth.hyperledger.org/docs/core/releases/1.0/introduction.html)
  - [Sawtooth REST API](https://sawtooth.hyperledger.org/docs/core/releases/1.0/rest_api.html)
- [Rustdocs for this application](#Building-The-Rustdocs)

---
