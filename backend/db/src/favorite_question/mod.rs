//! Module containing all structures and functions required for favorite question related database functionality.
pub mod db_types;
pub mod fake_impl;
pub mod interface;
pub mod pg_impl;
#[cfg(test)]
mod tests;
