pub mod types;
pub mod utils;
pub mod processor;
pub mod instructions;
pub mod state;
pub mod errors;

#[macro_use]
extern crate lazy_static;

solana_program::declare_id!("J7NojoAXWQhC58zEbT6rEwYxrRYDaQkKBrXZgUXJXfdr");

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
