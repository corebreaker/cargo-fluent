mod config;
mod cmd_scan;
pub mod arg_validators;

pub mod commands {
    pub use super::cmd_scan::command as cmd_scan;
}