// Copyright (c) 2018 Joseph Venetucci
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE.md in the
// source distribution of this software for license terms.

//! `rusty-saw-view` is a Rust program that helps visualize blockchain data from 
//! [Hyperledger Sawtooth](https://www.hyperledger.org/projects/sawtooth). Sawtooth is a blockchain framework that 
//! provides a REST API to query information such as the current state of the ledger, individual blocks and
//! transaction, and the entire blockchain itself. Requesting this information from the respective HTTP endpoints returns a 
//! JSON formatted block of data. 
//! 
//! From my own experience, Sawtooth is great for including a blockchain based distributed ledger into an application. However 
//! I learned that explaining how transactions were being made and how the blockchain worked was difficult with people who were 
//! unfamiliar with the concept of a blockchain. For transactions and blockchains that were simple I could easily draw it on a 
//! whiteboard to help explain it. As the transactions and application got more complex, so did the difficulty in explaining what 
//! was going on. Showing the JSON data wasn't really to helpful because (1) the data in each block is serialized and encoded (2) 
//! and while it's in human readable JSON format, it's hard to connect each block/transaction to each other when it's just one 
//! stream of text.
//! 
//! `rusty-saw-view` attempts to solve this problem by contacting the HTTP endpoints of a Sawtooth node and requesting the JSON data. 
//! It's job is to parse and visually display this data in a way thats easy to understand and explain.
//! 
//! ## Usage
//! Running the command `cargo run -- -help` will display the folowing:
//! ```bash
//! USAGE:
//!     rusty-saw-view [FLAGS] <endpoint> <method> <source> <location>
//! 
//! FLAGS:
//!     -f, --full-addr    Prints out full addresses & PubKeys
//!     -g, --genesis      Prints out the settings state or genesis block depending on the context
//!     -h, --help         Prints help information
//!     -n, --no-color     Prints without colored text. Use for piping to file
//!     -V, --version      Prints version information
//! ARGS:
//!     <endpoint>    From which endpoint is the data coming from? [possible values: state, blocks]
//!     <method>      What deserialization method to use? [possible values: cbor, json, custom]
//!     <source>      Where is the data coming from? [possible values: file, url]
//!     <location>    File path, or URL to data
//! ```
//! 
//! ## Supported Deserialization Methods
//! Out of the box the following deserialization methods are supported:
//! - CBOR
//! - JSON
//! 
//! Since Sawtooth is modular by design, it allows developers to use whatever serialization scheme they want.
//! This made the development of this application difficult because it can't possibly support every scheme
//! out of the box. For methods not listed in the previous section, users will have to add it to a section of
//! the project in order to use it. If you want to use a custom method or one not supported
//! see the [parse_custom()](json_deserialize/fn.parse_custom.html) method
//! 
//! ## Additional Info
//! For details on how to use this crate, see the README located at the projects [Github page](https://github.com/jvenetucci/rusty-saw-view)
//! 
//! Contact the author:
//! 
//! Joseph Venetucci <venetuc@pdx.edu>

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;

extern crate serde;
extern crate colored;

pub mod json_structs;
pub mod json_reader;
pub mod json_deserialize;

use clap::{App, Arg};

use json_structs::json_blocks::{BlockData};
use json_structs::json_state::{StateData};
use json_reader::{read_block_data_from_file, read_state_data_from_file,
    read_block_data_from_endpoint, read_state_data_from_endpoint};

#[derive(Debug)]
enum DataTypes {
    Blocks(BlockData),
    State(StateData),
}

fn main() {
    let endpoints = ["state", "blocks",];
    let methods = ["cbor", "json", "custom",];
    let sources = ["file", "url",];

    // CLI clap configuration
    let matches = App::new("rusty-saw-view")
        .version(crate_version!())
        .author("Joseph Venetucci <venetuc@pdx.edu>")
        .about("An application for parsing and viewing blockchain data from Hyperledger Sawtooth")
        .arg(Arg::from_usage("<endpoint> 'From which endpoint is the data coming from?'")
            .possible_values(&endpoints))
        .arg(Arg::from_usage("<method> 'What deserialization method to use?'")
            .possible_values(&methods))
        .arg(Arg::from_usage("<source> 'Where is the data coming from?'")
            .possible_values(&sources))
        .arg(Arg::from_usage("<location> 'File path, or URL to data'"))
        .arg(Arg::from_usage("[no-color] -n --no-color 'Prints without colored text. Use for piping to file'"))
        .arg(Arg::from_usage("[full-addr] -f --full-addr 'Prints out full addresses & PubKeys'"))
        .arg(Arg::from_usage("[genesis] -g --genesis 'Prints out the settings state or genesis block depending on the context'"))
        .get_matches();


    // Create a tuple with the passed in (endpoint, source).
    // Safe to unwrap since these are required by clap
    let config = (matches.value_of("endpoint").unwrap(), matches.value_of("source").unwrap());

    // Do the same for options that will be passed to display
    let options = (matches.is_present("full-addr"), matches.is_present("genesis"), String::from(matches.value_of("method").unwrap()));

    let loc = matches.value_of("location").unwrap();

    // Read in the data to an enum.
    let data: DataTypes = match config {
        ("state", "file") => DataTypes::State(read_state_data_from_file(loc)),
        ("state", "url") => DataTypes::State(read_state_data_from_endpoint(loc)),
        ("blocks", "file") => DataTypes::Blocks(read_block_data_from_file(loc)),
        ("blocks", "url") => DataTypes::Blocks(read_block_data_from_endpoint(loc)),
        _ => panic!("This should be unreachable")
    };

    // Print out the data, selecting the correct method based on the no-color flag
    match (data, matches.is_present("no-color")) {
        (DataTypes::Blocks(block), true) => block.display_full_data_no_color(options),
        (DataTypes::Blocks(block), false) => block.display_full_data(options),
        (DataTypes::State(state), true) => state.display_full_data_no_color(options),
        (DataTypes::State(state), false) => state.display_full_data(options),
    }
}