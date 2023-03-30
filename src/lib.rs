//! A program for creating and managing stake pool lottos.


// The deployed program id.
solana_program::declare_id!("98iqnEfLWpWK69Yn7YAPnWvkCSZAUgkXeS4tciGjccHQ");

#[cfg(not(feature = "no-entrypoint"))]
pub mod check;
pub mod create;
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
