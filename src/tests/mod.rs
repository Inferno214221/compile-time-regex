#![allow(clippy::module_inception)]
use ct_regex::*;

mod anchored;
mod capturing;
mod lazy;
mod literal;
mod quantified;
mod zero_width;

// TODO: test flags
