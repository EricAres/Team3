//! Substrate Node Template CLI library.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;

pub use substrate_cli::{VersionInfo, IntoExit, error};

fn main() {
	let version = VersionInfo {
		name: "Substrate Node",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
<<<<<<< HEAD
		executable_name: "kim-jong-un",
		author: "longjianjin",
		description: "Kim_Jong_un",
=======
		executable_name: "substrate-kitties",
		author: "Bryan Chen",
		description: "substrate-kitties",
>>>>>>> 45da911e27a064a32e24c60aafd7ec0f807773bd
		support_url: "support.anonymous.an",
	};

	if let Err(e) = cli::run(::std::env::args(), cli::Exit, version) {
		eprintln!("Fatal error: {}\n\n{:?}", e, e);
		std::process::exit(1)
	}
}
