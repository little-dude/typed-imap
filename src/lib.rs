#![allow(warnings)]
extern crate rand;
extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;

pub mod imaperror;

#[macro_export]
macro_rules! try_imap {
    ($old_state:expr, $expr:expr) => (match $expr {
        Result::Ok(val) => val,
        Result::Err(err) => {
            return Result::Err(($old_state, From::from(err)))
        }
    })
}

pub mod validate_helpers;
pub mod response;
pub mod parser;
pub mod client;
pub mod connection;
pub mod mailbox;
