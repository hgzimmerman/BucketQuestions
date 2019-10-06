//! All database queries directly related to users are contained within this module.

pub mod db_types;
pub mod fake_impl;
pub mod interface;
pub mod pg_impl;
#[cfg(test)]
mod tests;
pub mod wire_types;
