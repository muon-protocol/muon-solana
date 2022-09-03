pub mod types;
pub mod utils;
pub mod processor;
pub mod instructions;
pub mod state;
pub mod errors;

solana_program::declare_id!("3rTkQmLuC7LGy5uztxgQuojQVGxadPvG6SF28XxZTpku");

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
