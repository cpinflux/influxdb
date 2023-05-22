//! InfluxDB IOx implementation of FlightSQL

#![deny(rustdoc::broken_intra_doc_links, rust_2018_idioms)]
#![warn(
    clippy::clone_on_ref_ptr,
    clippy::dbg_macro,
    clippy::explicit_iter_loop,
    // See https://github.com/influxdata/influxdb_iox/pull/1671
    clippy::future_not_send,
    clippy::todo,
    clippy::use_self,
    missing_debug_implementations,
    // Allow missing docs - there's lots missing!
)]

mod cmd;
mod error;
mod get_catalogs;
mod get_db_schemas;
mod get_tables;
mod planner;
mod sql_info;
mod xdbc_type_info;

pub use cmd::{FlightSQLCommand, PreparedStatementHandle};
pub use error::{Error, Result};
pub use planner::FlightSQLPlanner;
