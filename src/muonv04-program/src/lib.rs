pub mod types;
pub mod utils;
pub mod processor;
pub mod instructions;
pub mod state;
pub mod errors;

solana_program::declare_id!("8T4P7taikE3EfoEogURTdF8TDpSEVk74KuQovw7Yz58t");

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
