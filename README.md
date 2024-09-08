# ROpts
**NOTE:** *I wrote this library for learning purposes. It may not be completely thought out and error free*. ***Use at Your Own Risk.***

[![Crates.io](https://img.shields.io/crates/v/ropts.svg)](https://crates.io/crates/ropts)
![CC0 licensed](https://img.shields.io/github/license/StevenCyb/ropts)
[![Tests](https://github.com/StevenCyb/ropts/actions/workflows/tests.yml/badge.svg)](https://github.com/StevenCyb/ropts/actions/workflows/tests.yml)
[![Clippy](https://github.com/StevenCyb/ropts/actions/workflows/lint.yml/badge.svg)](https://github.com/StevenCyb/ropts/actions/workflows/lint.yml)
[![Rustfmt](https://github.com/StevenCyb/ropts/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/StevenCyb/ropts/actions/workflows/rustfmt.yml)
[![Release](https://github.com/StevenCyb/ropts/actions/workflows/release.yml/badge.svg)](https://github.com/StevenCyb/ropts/actions/workflows/release.yml)

ROpts is a simple and intuitive option parsing library for Rust, designed to handle both command line arguments and environment variables. This library aims to make it easy to integrate option parsing into your Rust projects.

## Example
```rust
// Import the necessary modules
use std::env;
use ropts::{compose::Compose, error::Error, options::ValueOption, options::ValuesOption};

fn main() {
    // Collect the command line arguments and environment variables
    let args = env::args().collect::<Vec<String>>();
    let envs: Vec<(String, String)> = env::vars().collect();

    // Define the variables to store the parsed values
    let mut name = None::<String>;
    let mut age = None::<u8>;
    let mut employed: Option<bool> = Option::<bool>::None;
    let mut skills: Option<Vec<String>> = None;

    // Define the options
    let name_option = ValueOption::new(&mut name, "Your name")
        .required()
        .env("DEMO_NAME")
        .short_arg('n')
        .long_arg("name")
        .additional_eval(|s| {
            if s.len() < 3 {
                return Err(Error::Validation(
                    "Name must be at least 3 characters".into(),
                ));
            }
            Ok(())
        });
    let age_option = ValueOption::new(&mut age, "Your age")
        .env("DEMO_AGE")
        .short_arg('a')
        .long_arg("age")
        .additional_eval(|a| {
            if *a < 18 {
                return Err(Error::Validation(
                    "You must be at least 18 years old".into(),
                ));
            }
            Ok(())
        });
    let employed_option = ValueOption::new(&mut employed, "Are you employed?")
        .env("DEMO_EMPLOYED")
        .short_arg('e')
        .long_arg("employed")
        .default(true);
    let skills_option = ValuesOption::new(&mut skills, "Your skills")
        .env("DEMO_SKILLS")
        .short_arg('s')
        .long_arg("skills");

    // Compose the options and parse the command line arguments
    let result = Compose::new()
        .args(args.iter().skip(1).cloned())
        .envs(envs.into_iter())
        .help(|s| println!("{}", s))
        .add(name_option)
        .add(age_option)
        .add(employed_option)
        .add(skills_option)
        .parse();

    // Error handling
    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    // Use the parsed values
    println!("Hello, {}!", name.unwrap());
    println!("You are {} years old", age.unwrap());
    println!("Are you employed? {}", employed.unwrap());
    if skills.is_some() {
        println!("Your skills are: {:?}", skills.unwrap());
    } else {
        println!("You have no skills");
    }
}
```

## Installation

Add `ropts` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
ropts = "0.0.1"
```
