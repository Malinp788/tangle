//! Substrate Node Template CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod benchmarking;
mod cli;
mod command;
mod distributions;
mod eth;
mod rpc;
mod signer_cli;
mod testnet_fixtures;
mod utils;

fn main() -> sc_cli::Result<()> {
	command::run()
}
