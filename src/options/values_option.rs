use crate::error::Error;
use crate::options::utils::{convert, AllowedTypes};
use crate::options::{OptionBase, OptionBaseAttributes};
use std::any::TypeId;
use std::collections::HashMap;

pub struct ValuesOption<'a, T: AllowedTypes> {
    base: OptionBaseAttributes<'a, Vec<T>>,
}

impl<T: AllowedTypes> OptionBase for ValuesOption<'_, T> {
    fn parse_env(&mut self, envs: &HashMap<String, String>) {
        if let Some(env_key) = &self.base.env_key {
            if let Some(value) = envs.get(env_key) {
                // TODO
                // *self.base.value = Some(convert(value).unwrap());
            }
        }
    }

    fn parse_args(&mut self, args: &[String]) {
        // if let Some(long) = &self.base.long_arg {
        //     let long_flag = format!("--{}", long);
        //     if let Some(index) = args.iter().position(|arg| arg == &long_flag) {
        //         if let Some(value) = args.get(index + 1) {
        //             *self.base.value = Some(convert(value).unwrap());
        //         }
        //     }
        // }

        // if let Some(short) = self.base.short_arg {
        //     let short_flag = format!("-{}", short);
        //     if let Some(index) = args.iter().position(|arg| arg == &short_flag) {
        //         if let Some(value) = args.get(index + 1) {
        //             *self.base.value = Some(convert(value).unwrap());
        //         }
        //     }
        // }
    }

    fn eval(&mut self) -> Result<(), Error> {
        self.base.eval()?;
        Ok(())
    }

    fn help(&self) -> String {
        self.base.help()
    }
}

impl<'a, T: AllowedTypes + 'static> ValuesOption<'a, T> {
    pub fn new(value: &'a mut Option<Vec<T>>, description: &str) -> Self {
        ValuesOption {
            base: OptionBaseAttributes {
                description: description.into(),
                env_key: None,
                long_arg: None,
                short_arg: None,
                required: false,
                default: None,
                value,
                additional_eval: None,
                type_id: TypeId::of::<Vec<T>>(),
            },
        }
    }

    pub fn env(mut self, key: &str) -> Self {
        self.base.env_key = Some(key.into());
        self
    }

    pub fn long_arg(mut self, arg: &str) -> Self {
        self.base.long_arg = Some(arg.into());
        self
    }

    pub fn short_arg(mut self, arg: char) -> Self {
        self.base.short_arg = Some(arg);
        self
    }

    pub fn required(mut self) -> Self {
        self.base.required = true;
        self
    }

    pub fn default(mut self, value: Vec<T>) -> Self {
        self.base.default = Some(value.into());
        self
    }

    pub fn additional_eval(mut self, eval_fkt: impl Fn(&Vec<T>) -> Result<(), Error> + 'a) -> Self {
        self.base.additional_eval = Some(Box::new(eval_fkt));
        self
    }
}
