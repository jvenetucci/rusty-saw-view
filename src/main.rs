//! This is documentation for the `rusty-saw-view` crate.
//! 
//! ## Supported Deserialization Schemes

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
    let endpoints = ["state", "blocks"];
    let methods = ["cbor",];
    let sources = ["file", "url"];

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