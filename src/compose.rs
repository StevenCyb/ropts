use crate::error::Error;
use crate::options::OptionBase;
use std::collections::HashMap;

pub struct Compose<'a> {
    options: Vec<Box<dyn OptionBase + 'a>>,
    envs: HashMap<String, String>,
    args: Vec<String>,
    print_fn: Option<Box<dyn Fn(String) + 'a>>,
}

impl<'a> Compose<'a> {
    pub fn new() -> Self {
        Compose {
            options: Vec::new(),
            envs: HashMap::new(),
            args: Vec::new(),
            print_fn: None,
        }
    }

    pub fn add<T: OptionBase + 'a>(mut self, option: T) -> Self {
        self.options.push(Box::new(option));
        self
    }

    pub fn envs(mut self, envs: impl Iterator<Item = (String, String)>) -> Self {
        self.envs = envs.collect();
        self
    }

    pub fn args(mut self, args: impl Iterator<Item = String>) -> Self {
        self.args = args.collect();
        self
    }

    fn envs_parse(&mut self) {
        for option in &mut self.options {
            option.parse_env(&self.envs);
        }
    }

    fn args_parse(&mut self) {
        for arg in &self.args {
            if arg == "--help" || arg == "-h" {
                if let Some(print_fn) = &self.print_fn {
                    let mut help: String = "Usage: <program> [options]\n\n".to_string();

                    if self.options.len() > 0 {
                        help.push_str("Options:\n");
                        for option in &self.options {
                            help.push_str(format!(" - {}\n", option.help()).as_str());
                        }
                    }
                    print_fn(help);
                }
                break;
            }
        }
        for option in &mut self.options {
            option.parse_args(&self.args);
        }
    }

    pub fn parse(mut self) -> Result<(), Error> {
        if !self.envs.is_empty() {
            self.envs_parse();
        }
        if !self.args.is_empty() {
            self.args_parse();
        }

        for option in &mut self.options {
            option.eval()?;
        }

        Ok(())
    }

    pub fn help(mut self, print_fn: impl Fn(String) + 'a) -> Self {
        self.print_fn = Some(Box::new(print_fn));
        self
    }
}

#[cfg(test)]
mod compose_tests {
    use crate::error::Error;
    use crate::options::OptionBase;
    use std::collections::HashMap;

    pub struct MockOption {
        expected_env: HashMap<String, String>,
        expected_args: Vec<String>,
        eval_result: Result<(), Error>,
    }
    impl OptionBase for MockOption {
        fn parse_env(&mut self, envs: &std::collections::HashMap<String, String>) {
            assert_eq!(self.expected_env, *envs);
        }
        fn parse_args(&mut self, _args: &[String]) {
            assert_eq!(self.expected_args, _args);
        }
        fn eval(&mut self) -> Result<(), Error> {
            self.eval_result.clone()
        }
        fn help(&self) -> String {
            "MockOption".to_string()
        }
    }

    #[test]
    fn parsing() {
        let args = vec!["--test".to_string(), "test_value".to_string()];
        let mut env = HashMap::new();
        env.insert("TEST_ENV".to_string(), "test_value".to_string());

        let mut compose = super::Compose::new();
        assert_eq!(compose.options.len(), 0);
        assert!(compose.envs.is_empty());
        assert!(compose.args.is_empty());

        compose = compose.envs(env.clone().into_iter());
        compose = compose.args(args.clone().into_iter());
        assert_eq!(compose.envs, env);
        assert_eq!(compose.args, args);

        compose = compose.add(MockOption {
            expected_env: env.clone(),
            expected_args: args.clone(),
            eval_result: Ok(()),
        });
        assert_eq!(compose.options.len(), 1);

        let result: Result<(), crate::error::Error> = compose.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn parse_propagate_error() {
        let args = vec!["--test".to_string(), "test_value".to_string()];

        let mut compose = super::Compose::new();
        compose = compose.args(args.clone().into_iter());
        compose = compose.add(MockOption {
            expected_env: HashMap::new(),
            expected_args: args.clone(),
            eval_result: Err(Error::Validation("test".to_string())),
        });

        let result: Result<(), crate::error::Error> = compose.parse();
        assert!(result.is_err());
    }
}
