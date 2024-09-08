use ropts::{compose::Compose, error::Error, options::ValueOption, options::ValuesOption};
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let envs: Vec<(String, String)> = env::vars().collect();

    let mut name = None::<String>;
    let mut employed: Option<bool> = Option::<bool>::None;
    let mut skills: Option<Vec<String>> = None;

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
    let employed_option = ValueOption::new(&mut employed, "Are you employed?")
        .env("DEMO_EMPLOYED")
        .short_arg('e')
        .long_arg("employed")
        .default(true);
    let skills_option = ValuesOption::new(&mut skills, "Your skills")
        .env("DEMO_SKILLS")
        .short_arg('s')
        .long_arg("skills");

    let result = Compose::new()
        .args(args.iter().skip(1).cloned())
        .envs(envs.into_iter())
        .help(|s| println!("{}", s))
        .add(name_option)
        .add(employed_option)
        .add(skills_option)
        .parse();

    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    println!("Hello, {}!", name.unwrap());
    println!("Are you employed? {}", employed.unwrap());
    // println!("Your skills are: {:?}", skills.unwrap());
}
