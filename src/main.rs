#[macro_use]
extern crate serde_derive;

mod actions;
mod config;
mod github;
mod gitlab;

use cargo_toml::Manifest;
use clap::{App, Arg, SubCommand};
use dockerfile_parser::Dockerfile;

static PROJECT_NAME: &str = "mcai-workers";

#[derive(Debug)]
pub struct Repository {
  name: String,
  cargo_manifest: Option<Manifest>,
  dockerfile: Option<Dockerfile>,
}

fn main() {
  let github_token_arg = Arg::with_name("github-token")
    .long("github-token")
    .env("GITHUB_TOKEN");

  let gitlab_token_arg = Arg::with_name("gitlab-token")
    .long("gitlab-token")
    .env("GITLAB_TOKEN");

  let matches = App::new(PROJECT_NAME)
    .subcommand(
      SubCommand::with_name("register")
        .about("Register a new repository to list of current workers")
        .version("0.1")
        .arg(
          Arg::with_name("repository")
            .short("r")
            .long("repository")
            .takes_value(true)
            .required(true),
        )
        .arg(
          Arg::with_name("provider")
            .short("p")
            .long("provider")
            .takes_value(true)
            .possible_values(&["github", "gitlab"])
            .required(true),
        )
        .arg(
          Arg::with_name("manifest")
            .short("m")
            .long("manifest")
            .takes_value(true)
            .multiple(true),
        )
        .arg(
          Arg::with_name("dockerfile")
            .short("d")
            .long("dockerfile")
            .takes_value(true)
            .multiple(true),
        ),
    )
    .subcommand(
      SubCommand::with_name("unregister")
        .about("Unregister a repository")
        .version("0.1")
        .arg(
          Arg::with_name("repository")
            .short("r")
            .long("repository")
            .takes_value(true)
            .required(true),
        )
        .arg(
          Arg::with_name("provider")
            .short("p")
            .long("provider")
            .takes_value(true)
            .possible_values(&["github", "gitlab"])
            .required(true),
        ),
    )
    .subcommand(
      SubCommand::with_name("show")
        .about("Display stored configuration")
        .version("0.1"),
    )
    .subcommand(
      SubCommand::with_name("fetch")
        .about("Update local cache from repositories")
        .version("0.1")
        .arg(github_token_arg.clone())
        .arg(gitlab_token_arg.clone()),
    )
    .subcommand(
      SubCommand::with_name("list")
        .about("List status of registered workers")
        .version("0.1")
        .arg(github_token_arg)
        .arg(gitlab_token_arg)
        .arg(
          Arg::with_name("dependencies")
            .short("d")
            .long("dependencies"),
        )
        .arg(
          Arg::with_name("exclude-sdk-versions")
            .short("e")
            .long("exclude-sdk-versions")
            .takes_value(true),
        ),
    )
    .get_matches();

  let mut cfg: config::McaiWorkersConfig = confy::load(PROJECT_NAME).unwrap();

  if let Some(matches) = matches.subcommand_matches("register") {
    actions::register(&mut cfg, matches);
    return;
  }

  if let Some(matches) = matches.subcommand_matches("unregister") {
    actions::unregister(&mut cfg, matches);
    return;
  }

  if matches.subcommand_matches("show").is_some() {
    actions::show(&cfg);
    return;
  }

  if let Some(matches) = matches.subcommand_matches("fetch") {
    actions::fetch(&mut cfg, matches);
    return;
  }

  if let Some(matches) = matches.subcommand_matches("list") {
    actions::list(&cfg, matches);
    return;
  }
}
