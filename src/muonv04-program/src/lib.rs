pub mod types;
pub mod utils;
pub mod processor;
pub mod instructions;
pub mod state;
pub mod errors;

solana_program::declare_id!("6XFPBb6wj4NemqgQN7pP3GwHPCjpFCv6rx7ZzzTfSYac");

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
