use ropts::{compose::Compose, error::Error, options::ValueOption};
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let envs: Vec<(String, String)> = env::vars().collect();

    let mut name = None::<String>;

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

    let result = Compose::new()
        .args(args.iter().skip(1).cloned())
        .envs(envs.into_iter())
        .help(|s| println!("{}", s))
        .add(name_option)
        .parse();

    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    println!("Hello, {}!", name.unwrap());
}
