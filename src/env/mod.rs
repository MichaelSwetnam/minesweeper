use std::{env, str::FromStr, fmt::Debug};

pub fn acquire_string(name: &'static str) -> String {
    env::var(name).expect(&format!("Expected environment variable '{}' which was not set.", name))
}

pub fn acquire_num<T>(name: &'static str) -> T
where
    T: FromStr,
    T::Err: Debug
{
    acquire_string(name).parse::<T>().expect(&format!("Expected environment variable '{name}' to be a parsable u32"))
}