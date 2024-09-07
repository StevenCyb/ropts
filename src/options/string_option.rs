use crate::error::Error;
use crate::options::{OptionBase, OptionBaseAttributes};
use std::collections::HashMap;

pub struct StringOption<'a> {
    base: OptionBaseAttributes<'a, String>,
}

// Implement OptionBase for StringOption
impl OptionBase for StringOption<'_> {
    fn parse_env(&mut self, envs: &HashMap<String, String>) {
        if let Some(env_key) = &self.base.env_key {
            if let Some(val) = envs.get(env_key) {
                *self.base.value = Some(val.clone());
            }
        }
    }

    fn parse_args(&mut self, args: &[String]) {
        if let Some(long) = &self.base.long_arg {
            let long_flag = format!("--{}", long);
            if let Some(idx) = args.iter().position(|arg| arg == &long_flag) {
                if let Some(val) = args.get(idx + 1) {
                    *self.base.value = Some(val.clone());
                }
            }
        }

        if let Some(short) = self.base.short_arg {
            let short_flag = format!("-{}", short);
            if let Some(idx) = args.iter().position(|arg| arg == &short_flag) {
                if let Some(val) = args.get(idx + 1) {
                    *self.base.value = Some(val.clone());
                }
            }
        }
    }

    fn eval(&mut self) -> Result<(), Error> {
        self.base.eval()?;
        Ok(())
    }

    fn help(&self) -> String {
        self.base.help()
    }
}

impl<'a> StringOption<'a> {
    pub fn new(value: &'a mut Option<String>, description: &str) -> Self {
        StringOption {
            base: OptionBaseAttributes {
                description: description.into(),
                env_key: None,
                long_arg: None,
                short_arg: None,
                required: false,
                default: None,
                value,
                additional_eval: None,
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

    pub fn default(mut self, value: &str) -> Self {
        self.base.default = Some(value.into());
        self
    }

    pub fn additional_eval(mut self, eval_fkt: impl Fn(&String) -> Result<(), Error> + 'a) -> Self {
        self.base.additional_eval = Some(Box::new(eval_fkt));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::options::{OptionBase, StringOption};

    #[test]
    fn parse_env_existing() {
        let mut value = None::<String>;
        {
            let mut opt = StringOption::new(&mut value, "test").env("TEST_ENV");
            let mut env = std::collections::HashMap::new();
            env.insert("TEST_ENV".to_string(), "test_value".to_string());
            opt.parse_env(&env);
        }
        assert_eq!(value, Some("test_value".to_string()));
    }

    #[test]
    fn parse_env_missing() {
        let mut value = None::<String>;
        {
            let mut opt = StringOption::new(&mut value, "test").env("TEST_ENV");
            let env = std::collections::HashMap::new();
            opt.parse_env(&env);
        }
        assert_eq!(value, None);
    }

    #[test]
    fn parse_args_long_existing() {
        let mut value = None::<String>;
        {
            let mut opt = StringOption::new(&mut value, "test").long_arg("test");
            opt.parse_args(&["--test".to_string(), "test_value".to_string()]);
        }
        assert_eq!(value, Some("test_value".to_string()));
    }

    #[test]
    fn parse_args_long_missing() {
        let mut value = None::<String>;
        {
            let mut opt = StringOption::new(&mut value, "test").long_arg("test");
            opt.parse_args(&["--not_test".to_string(), "test_value".to_string()]);
        }
        assert_eq!(value, None);
    }

    #[test]
    fn parse_args_short_existing() {
        let mut value = None::<String>;
        {
            let mut opt = StringOption::new(&mut value, "test").short_arg('t');
            opt.parse_args(&["-t".to_string(), "test_value".to_string()]);
        }
        assert_eq!(value, Some("test_value".to_string()));
    }

    #[test]
    fn parse_args_short_missing() {
        let mut value = None::<String>;
        {
            let mut opt = StringOption::new(&mut value, "test").short_arg('t');
            opt.parse_args(&["-n".to_string(), "test_value".to_string()]);
        }
        assert_eq!(value, None);
    }

    #[test]
    fn parse_setters_unused() {
        let mut value = None::<String>;
        let opt = StringOption::new(&mut value, "my description");

        assert_eq!(opt.base.description, "my description");
        assert!(opt.base.additional_eval.is_none());
        assert_eq!(opt.base.long_arg, None);
        assert_eq!(opt.base.short_arg, None);
        assert_eq!(opt.base.required, false);
        assert!(opt.base.value.is_none());
        assert_eq!(opt.base.default, None);
        assert!(opt.base.additional_eval.as_deref().is_none());
    }

    #[test]
    fn parse_setters_used() {
        let mut value = None::<String>;
        let eval = |s: &String| {
            assert_eq!(s, "test_input");
            Ok(())
        };

        let opt = StringOption::new(&mut value, "my description")
            .env("TEST_ENV")
            .long_arg("test")
            .short_arg('t')
            .required()
            .default("default_value")
            .additional_eval(eval);

        assert_eq!(opt.base.description, "my description");
        assert!(opt.base.additional_eval.is_some());
        assert_eq!(opt.base.long_arg, Some("test".to_string()));
        assert_eq!(opt.base.short_arg, Some('t'));
        assert_eq!(opt.base.required, true);
        assert!(opt.base.value.is_none());
        assert_eq!(opt.base.default, Some("default_value".to_string()));

        // Test the eval closure by calling it with "test_input"
        if let Some(eval_fn) = opt.base.additional_eval.as_deref() {
            assert!(eval_fn(&"test_input".to_string()).is_ok());
        } else {
            panic!("Eval function was not set.");
        }
    }

    #[test]
    fn eval_not_set() {
        let mut value = None::<String>;
        {
            let mut opt = StringOption::new(&mut value, "test").env("key");
            assert!(opt.eval().is_ok());
        }
        assert_eq!(value, None);
    }

    #[test]
    fn eval_set() {
        let mut value = Some("test_value".to_string());
        {
            let mut opt = StringOption::new(&mut value, "test").env("key");
            assert!(opt.eval().is_ok());
        }
        assert_eq!(value, Some("test_value".to_string()));
    }

    #[test]
    fn eval_use_default() {
        let mut value = None::<String>;
        {
            let mut opt = StringOption::new(&mut value, "test")
                .default("default_value")
                .env("key");
            assert!(opt.eval().is_ok());
        }
        assert_eq!(value, Some("default_value".to_string()));
    }

    #[test]
    fn eval_required_set() {
        let mut value = Some("test_value".to_string());
        let mut opt = StringOption::new(&mut value, "test").required().env("key");
        assert!(opt.eval().is_ok());
    }

    #[test]
    fn eval_required_not_set_and_formatted_error() {
        let mut value = None::<String>;
        let mut opt = StringOption::new(&mut value, "test").required();
        assert!(opt.eval().is_err());

        opt = opt.env("TEST_ENV");
        assert_eq!(
            opt.eval().unwrap_err().to_string(),
            Error::Validation("{TEST_ENV} is required".into()).to_string()
        );
    }

    #[test]
    fn eval_validation_passed() {
        let mut value = Some("test_value".to_string());
        let eval = |s: &String| {
            assert_eq!(s, "test_value");
            Ok(())
        };
        let mut opt = StringOption::new(&mut value, "test")
            .additional_eval(eval)
            .env("key");
        assert!(opt.eval().is_ok());
    }

    #[test]
    fn eval_validation_failed() {
        let mut value = Some("test_value".to_string());
        let eval = |s: &String| {
            assert_eq!(s, "test_value");
            Err(Error::Validation("validation failed".into()))
        };
        let mut opt = StringOption::new(&mut value, "test").additional_eval(eval);
        assert!(opt.eval().is_err());
    }

    #[test]
    fn help_no_env_only() {
        let mut value = None::<String>;
        let opt = StringOption::new(&mut value, "my description").env("ENV_KEY");
        assert_eq!(opt.help(), "ENV:ENV_KEY  - my description");
    }

    #[test]
    fn help_no_short_arg_only() {
        let mut value = None::<String>;
        let opt = StringOption::new(&mut value, "my description").short_arg('i');
        assert_eq!(opt.help(), "ARGS:-i - my description");
    }

    #[test]
    fn help_no_long_arg_only() {
        let mut value = None::<String>;
        let opt = StringOption::new(&mut value, "my description").long_arg("input");
        assert_eq!(opt.help(), "ARGS:--input - my description");
    }

    #[test]
    fn help_all_options() {
        let mut value = None::<String>;
        let opt = StringOption::new(&mut value, "my description")
            .env("ENV_KEY")
            .short_arg('i')
            .long_arg("input");
        assert_eq!(opt.help(), "ENV:ENV_KEY ARGS:-i,--input - my description");
    }
}
