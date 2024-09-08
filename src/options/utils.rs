use crate::error::Error;
use std::any::Any;
use std::fmt;
use std::str::FromStr;

pub trait AllowedTypes: fmt::Debug + Clone + FromStr + Any {
    fn as_any(&self) -> &dyn Any;
}

impl AllowedTypes for String {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AllowedTypes for i8 {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AllowedTypes for i16 {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AllowedTypes for i32 {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AllowedTypes for i64 {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AllowedTypes for i128 {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AllowedTypes for u8 {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AllowedTypes for u16 {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AllowedTypes for u32 {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AllowedTypes for u64 {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AllowedTypes for u128 {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AllowedTypes for f32 {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AllowedTypes for f64 {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AllowedTypes for char {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AllowedTypes for bool {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn convert<T: AllowedTypes>(value: &str) -> Result<T, Error> {
    value.parse().map_err(|_| {
        Error::Parsing(format!(
            "Error converting from {:?} to {}",
            value,
            std::any::type_name::<T>()
        ))
    })
}
