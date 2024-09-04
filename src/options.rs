use std::collections::HashMap;
use std::fmt;
pub mod string_option;
use crate::error::Error;
pub use string_option::StringOption;

pub struct OptionBaseAttributes<'a, T>
where
    T: fmt::Display + Clone,
{
    description: String,
    env_key: Option<String>,
    long_arg: Option<String>,
    short_arg: Option<char>,
    required: bool,
    default: Option<T>,
    value: &'a mut Option<T>,
    additional_eval: Option<Box<dyn Fn(&T) -> Result<(), Error>>>,
}

pub trait OptionBase {
    fn parse_env(&mut self, envs: &HashMap<String, String>);
    fn parse_args(&mut self, args: &[String]);
    fn eval(&mut self) -> Result<(), Error>;
    fn help(&self) -> String;
}

impl<'a, T> OptionBaseAttributes<'a, T>
where
    T: fmt::Display + Clone,
{
    fn identifier(&self) -> String {
        let mut identifier = "{".to_string();

        if let Some(env_key) = &self.env_key {
            identifier.push_str(&format!("{}, ", env_key));
        }
        if let Some(short_arg) = self.short_arg {
            identifier.push_str(&format!("-{}, ", short_arg));
        }
        if let Some(long_arg) = &self.long_arg {
            identifier.push_str(&format!("--{}, ", long_arg));
        }

        identifier = identifier.trim_end_matches(", ").to_string();
        identifier.push_str("}");
        identifier
    }
}

impl<'a, T> OptionBase for OptionBaseAttributes<'a, T>
where
    T: fmt::Display + Clone,
{
    fn parse_env(&mut self, _: &HashMap<String, String>) {
        panic!("Not implemented, needs to be handled by the option type");
    }

    fn parse_args(&mut self, _: &[String]) {
        panic!("Not implemented, needs to be handled by the option type");
    }

    fn eval(&mut self) -> Result<(), Error> {
        if self.env_key.is_none() && self.long_arg.is_none() && self.short_arg.is_none() {
            return Err(Error::Parsing("No identifier set".into()));
        }

        if self.required && self.value.is_none() {
            return Err(Error::Validation(format!(
                "{} is required",
                self.identifier()
            )));
        }
        if self.value.is_none() {
            *self.value = self.default.clone();
        }
        if self.additional_eval.is_some() {
            let validation_result =
                self.additional_eval.as_ref().unwrap()(&self.value.as_ref().unwrap());
            if validation_result.is_err() {
                return Err(Error::Validation(format!(
                    "{} failed validation: {}",
                    self.identifier(),
                    validation_result.unwrap_err()
                )));
            }
        }

        Ok(())
    }

    fn help(&self) -> String {
        let mut help = String::new();

        if let Some(env_key) = &self.env_key {
            help.push_str(&format!("ENV:{} ", env_key));
        }
        if self.long_arg.is_some() || self.short_arg.is_some() {
            help.push_str(&format!("ARGS:"));
            if let Some(short_arg) = self.short_arg {
                help.push_str(&format!("-{},", short_arg));
            }
            if let Some(long_arg) = &self.long_arg {
                help.push_str(&format!("--{},", long_arg));
            }
            help.pop();
        }

        if self.required {
            help.push_str("  Required\n");
        } else if let Some(default) = &self.default {
            help.push_str(&format!("  Default: {}\n", default));
        }

        help.push_str(&format!(" - {}", self.description));

        help
    }
}

#[cfg(test)]
mod option_base_tests {
    use super::*;

    #[test]
    fn identifier_env() {
        let mut value = None::<String>;
        let opt = OptionBaseAttributes::<String> {
            description: "".to_string(),
            env_key: Some("ENV_KEY".to_string()),
            long_arg: None,
            short_arg: None,
            required: false,
            default: None,
            value: &mut value,
            additional_eval: None,
        };
        assert_eq!(opt.identifier(), "{ENV_KEY}");
    }

    #[test]
    fn identifier_short_arg() {
        let mut value = None::<String>;
        let opt = OptionBaseAttributes::<String> {
            description: "".to_string(),
            env_key: None,
            long_arg: None,
            short_arg: Some('i'),
            required: false,
            default: None,
            value: &mut value,
            additional_eval: None,
        };
        assert_eq!(opt.identifier(), "{-i}");
    }

    #[test]
    fn identifier_long_arg() {
        let mut value = None::<String>;
        let opt = OptionBaseAttributes::<String> {
            description: "".to_string(),
            env_key: None,
            long_arg: Some("long_arg".to_string()),
            short_arg: None,
            required: false,
            default: None,
            value: &mut value,
            additional_eval: None,
        };
        assert_eq!(opt.identifier(), "{--long_arg}");
    }

    #[test]
    fn identifier_all() {
        let mut value = None::<String>;
        let opt = OptionBaseAttributes::<String> {
            description: "".to_string(),
            env_key: Some("ENV_KEY".to_string()),
            long_arg: Some("long_arg".to_string()),
            short_arg: Some('i'),
            required: false,
            default: None,
            value: &mut value,
            additional_eval: None,
        };
        assert_eq!(opt.identifier(), "{ENV_KEY, -i, --long_arg}");
    }

    #[test]
    fn help_env() {
        let mut value = None::<String>;
        let opt = OptionBaseAttributes::<String> {
            description: "my description".to_string(),
            env_key: Some("ENV_KEY".to_string()),
            long_arg: None,
            short_arg: None,
            required: false,
            default: None,
            value: &mut value,
            additional_eval: None,
        };
        assert_eq!(opt.help(), "ENV:ENV_KEY  - my description");
    }

    #[test]
    fn help_short_arg() {
        let mut value = None::<String>;
        let opt = OptionBaseAttributes::<String> {
            description: "my description".to_string(),
            env_key: None,
            long_arg: None,
            short_arg: Some('i'),
            required: false,
            default: None,
            value: &mut value,
            additional_eval: None,
        };
        assert_eq!(opt.help(), "ARGS:-i - my description");
    }

    #[test]
    fn help_long_arg() {
        let mut value = None::<String>;
        let opt = OptionBaseAttributes::<String> {
            description: "my description".to_string(),
            env_key: None,
            long_arg: Some("long_arg".to_string()),
            short_arg: None,
            required: false,
            default: None,
            value: &mut value,
            additional_eval: None,
        };
        assert_eq!(opt.help(), "ARGS:--long_arg - my description");
    }

    #[test]
    fn help_all() {
        let mut value = None::<String>;
        let opt = OptionBaseAttributes::<String> {
            description: "my description".to_string(),
            env_key: Some("ENV_KEY".to_string()),
            long_arg: Some("long_arg".to_string()),
            short_arg: Some('i'),
            required: false,
            default: None,
            value: &mut value,
            additional_eval: None,
        };
        assert_eq!(
            opt.help(),
            "ENV:ENV_KEY ARGS:-i,--long_arg - my description"
        );
    }

    #[test]
    fn eval_no_identifier() {
        let mut value = None::<String>;
        let mut opt = OptionBaseAttributes::<String> {
            description: "my description".to_string(),
            env_key: None,
            long_arg: None,
            short_arg: None,
            required: false,
            default: None,
            value: &mut value,
            additional_eval: None,
        };
        assert!(opt.eval().is_err());
    }

    #[test]
    fn eval_required() {
        let mut value = None::<String>;
        let mut opt = OptionBaseAttributes::<String> {
            description: "my description".to_string(),
            env_key: Some("ENV_KEY".to_string()),
            long_arg: None,
            short_arg: None,
            required: true,
            default: None,
            value: &mut value,
            additional_eval: None,
        };
        assert!(opt.eval().is_err());
    }

    #[test]
    fn eval_default() {
        let mut value = None::<String>;
        let mut opt = OptionBaseAttributes::<String> {
            description: "my description".to_string(),
            env_key: Some("ENV_KEY".to_string()),
            long_arg: None,
            short_arg: None,
            required: false,
            default: Some("default".to_string()),
            value: &mut value,
            additional_eval: None,
        };
        assert!(opt.eval().is_ok());
        assert_eq!(value, Some("default".to_string()));
    }

    #[test]
    fn eval_additional_eval() {
        let mut value = Some("value".to_string());
        let mut opt = OptionBaseAttributes::<String> {
            description: "my description".to_string(),
            env_key: Some("ENV_KEY".to_string()),
            long_arg: None,
            short_arg: None,
            required: false,
            default: None,
            value: &mut value,
            additional_eval: Some(Box::new(|_| Ok(()))),
        };
        assert!(opt.eval().is_ok());
    }

    #[test]
    fn eval_additional_eval_fail() {
        let mut value = Some("value".to_string());
        let mut opt = OptionBaseAttributes::<String> {
            description: "my description".to_string(),
            env_key: Some("ENV_KEY".to_string()),
            long_arg: None,
            short_arg: None,
            required: false,
            default: None,
            value: &mut value,
            additional_eval: Some(Box::new(|_| Err(Error::Validation("fail".into())))),
        };
        assert!(opt.eval().is_err());
    }
}
