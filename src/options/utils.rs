use crate::error::Error;
use std::fmt;
use std::str::FromStr;

pub trait AllowedTypes: fmt::Display + Clone + FromStr {}
impl AllowedTypes for String {}
impl AllowedTypes for i8 {}
impl AllowedTypes for i16 {}
impl AllowedTypes for i32 {}
impl AllowedTypes for i64 {}
impl AllowedTypes for i128 {}
impl AllowedTypes for u8 {}
impl AllowedTypes for u16 {}
impl AllowedTypes for u32 {}
impl AllowedTypes for u64 {}
impl AllowedTypes for u128 {}
impl AllowedTypes for f32 {}
impl AllowedTypes for f64 {}
impl AllowedTypes for char {}

pub fn convert<T: AllowedTypes>(value: &str) -> Result<T, Error> {
    value.parse().map_err(|_| {
        Error::Parsing(format!(
            "Error converting from {:?} to {}",
            value,
            std::any::type_name::<T>()
        ))
    })
}
