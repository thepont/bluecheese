//! Ports of the Java detectors. One module per code smell; see
//! `src/detectors/mod.rs` for the dispatch table.

pub mod comments;
pub mod cross_language_duplicates;
pub mod data_clumps;
pub mod dup_code;
pub mod long_method;
pub mod long_params;
pub mod message_chains;
pub mod shotgun;
pub mod spec_gen;
pub mod switch_stmts;
