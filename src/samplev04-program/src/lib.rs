//pub mod types;
pub mod processor;
pub mod instructions;
//pub mod state;
pub mod errors;

extern crate muonv04;

//#[macro_use]
//extern crate lazy_static;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
