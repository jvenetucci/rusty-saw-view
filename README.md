# rusty-saw-view
## Overview
rusty-saw-view is a Rust program that helps visualize blockchain data from [Hyperledger Sawtooth](https://www.hyperledger.org/projects/sawtooth). Sawtooth is a blockchain framework that provides a REST API to query information such as the current state of the ledger, individual blocks and transaction, and the entire blockchain itself. Requesting this information from the respective HTTP endpoints returns a JSON formatted block of data.

From my own experience, Sawtooth is great for including a blockchain based distributed ledger into an application. However I learned that explaining how transactions were being made and how the blockchain worked was difficult with people who were unfamiliar with the concept of a blockchain. For transactions and blockchains that were simple I could easily draw it on a whiteboard to help explain it. As the transactions and application got more complex, so did the difficulty in explaining what was going on. Showing the JSON data wasn't really to helpful because (1) the data in each block is serialized and encoded (2) and while it's in human readable JSON format, it's hard to connect each block/transaction to each other when it's just one stream of text.

rusty-saw-view attempts to solve this problem by contacting the HTTP endpoints of a Sawtooth node and requesting the JSON data. It's job is to parse and visually display this data in a way thats easy to understand and explain. 

---

## Table of Contents
- [Features & Requirements](#Features-&-Requirements)
- [Project Structure](#Project-&-Directory-Structure)
- [How to Build & Run](#How-to-Build-&-Run)
- [Usage Guide](#Usage-Guide)
- [Testing Tools](#Testing-Tools)
- [Licensing](#Licensing)
- [Authors](#Authors)
- [Acknowledgments](#Acknowledgments)
- [References](#References)

---

## Features & Requirements
- [ ] Parse blockchain data stored as JSON files.
- [ ] Parse blockchain data by requesting it from the HTTP Endpoints.
- [ ] Support multiple deserialization & decoding formats.
    - [ ] Basics like CBOR/Base64
    - [ ] Support for advanced and custom protocols like Google Protobuf
- [ ] Display parsed data through the command line in text format.
- [ ] Display parsed data with a GUI.
    - [ ] Ability to select each block/transaction to learn more about it.

---

## Project & Directory Structure
The root directory of the project contains the following folders:
- **src** -- Contains code for the application.
- **example-blockchain** -- Contains example JSON data for a simple Sawtooth blockchain.
- **test** -- Various test suites and integration for testing the application.

---

## How to Build & Run
This section will provide an overview of how to build & run the application. Rustc and [Cargo](https://doc.rust-lang.org/stable/cargo/) are required.

### Building & Running
Navigate to the root of the project directory and run the following:
```Bash
$ cargo build
$ cargo run
```
Alternatively, running `cargo run` by itself will both build and run the program.

### Teardown & Cleanup
To remove the compiled files from building the program run the following in the root of the project directory:
```Bash
$ cargo clean
```

---

## Usage Guide
Todo

---

## Testing Tools
Todo

---

## Licensing
Todo

---

## Authors
For any questions related to the project you can contact the author:
- Joseph Venetucci - <venetuc@pdx.edu>

---

## Acknowledgments
Todo

---

## References
Todo

---