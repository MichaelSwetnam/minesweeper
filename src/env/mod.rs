mod vars;
pub use vars::EnvVariable;

use std::{env, str::FromStr, fmt::Debug};

// Debug
#[cfg(debug_assertions)]
pub fn acquire_string(var: &EnvVariable) -> String {
    env::var(var.as_ref()).expect(&format!("Expected environment variable '{}' which was not set.", var.as_ref()))
}

// Release
#[cfg(not(debug_assertions))]
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub fn acquire_num<T>(var: EnvVariable) -> T
where
    T: FromStr,
    T::Err: Debug
{
    acquire_string(&var).parse::<T>().expect(&format!("Expected environment variable '{}' to be a parsable u32", var.as_ref()))
}