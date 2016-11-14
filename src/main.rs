extern crate clap;
extern crate time;
extern crate slack_hook;

use clap::{Arg, App, SubCommand};
use cuisinier::Cuisinier;

mod recette;
mod cuisinier;

fn main() {
    let directory_arg = Arg::with_name("DIRECTORY")
        .help("The directory of your web app")
        .required(true)
        .index(1);

    let git_arg = Arg::with_name("GIT_REPOSITORY")
        .help("Sets the input file to use")
        .required(true)
        .index(2);

    let matches = App::new("Laravel Facteur")
        .version("0.1")
        .author("rap2h <raphaelht@gmail.com>")
        .about("A laravel deployer written in rust")
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .arg(Arg::with_name("simulation")
            .short("s")
            .help("Just run a simulation"))
        .subcommand(SubCommand::with_name("init")
            .arg(&directory_arg)
            .arg(&git_arg)
            .about("Initialization of a Laravel application"))
        .subcommand(SubCommand::with_name("deploy")
            .arg(&directory_arg)
            .arg(&git_arg)
            .about("Deploy a Laravel application (previously initialized)"))
        .subcommand(SubCommand::with_name("rollback")
            .arg(&directory_arg)
            .about("Rollback to previous release"))
        .get_matches();

    match matches.subcommand() {
        ("init", Some(sub_m)) => {
            recette::init(
                Cuisinier::new(sub_m.value_of("DIRECTORY").unwrap().to_string())
                    .git(sub_m.value_of("GIT_REPOSITORY").unwrap())
            );
        },
        ("deploy", Some(sub_m)) => {
            recette::deploy(
                Cuisinier::new(sub_m.value_of("DIRECTORY").unwrap().to_string())
                    .git(sub_m.value_of("GIT_REPOSITORY").unwrap())
            );
        },
        ("rollback", Some(sub_m)) => {
            recette::rollback(
                Cuisinier::new(sub_m.value_of("DIRECTORY").unwrap().to_string())
            );
        },
        _ => {
            println!("Please select a command");
        },
    }
}