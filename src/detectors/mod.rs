//! Ports of the Java detectors. One module per code smell; see
//! `src/detectors/mod.rs` for the dispatch table.

pub mod alt_classes;
pub mod comments;
pub mod cross_language_duplicates;
pub mod data_class;
pub mod data_clumps;
pub mod divergent_change;
pub mod dup_code;
pub mod feature_envy;
pub mod incomplete_library_class;
pub mod intimacy;
pub mod large_class;
pub mod lazy_class;
pub mod long_method;
pub mod long_params;
pub mod message_chains;
pub mod middle_man;
pub mod parallel_hierarchy;
pub mod primitive_obsession;
pub mod refused_bequest;
pub mod shotgun;
pub mod spec_gen;
pub mod switch_stmts;
pub mod temp_field;
