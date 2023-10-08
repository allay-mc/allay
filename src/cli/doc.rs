use clap::{Arg, ArgMatches, Command};
use url::Url;

use crate::environment::Environment;

pub(crate) fn cmd() -> Command {
    Command::new("doc")
        .about("Browse the Allay documentation")
        .arg(
            Arg::new("search")
                .help("Search text")
                .required(false)
                .num_args(1..)
                .trailing_var_arg(true),
        )
}

pub(crate) fn run(matches: &ArgMatches, _env: &mut Environment) {
    let docs_url = "https://allay-mc.github.io/docs/";
    let search: Option<Vec<_>> = matches.get_many::<String>("search").map(|x| x.collect());
    let url = match search {
        Some(query) => {
            let query = query.iter();
            let mut s = String::new();
            for word in query {
                s.push_str(word);
                s.push(' ');
            }
            s.pop();
            Url::parse_with_params(docs_url, &[("search", s)]).unwrap()
        }
        None => Url::parse(docs_url).unwrap(),
    };
    let url: &str = url.as_str();
    match open::that(url) {
        Ok(()) => log::debug!("Opened '{}' successfully", url),
        Err(e) => {
            log::error!("Error while opening '{}': {}", url, e);
            println!("{}", url);
        }
    }
}
