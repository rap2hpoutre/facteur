extern crate clap;
extern crate time;

use clap::{Arg, App, SubCommand};
use facteur::Facteur;

mod task;
mod facteur;

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
        .arg(Arg::with_name("pretend")
            .short("p")
            .long("pretend")
            .help("Run a simulation (don't do anything)"))
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


    let (sub_c, sub_m) = matches.subcommand();
    match vec!["init", "deploy", "rollback"].iter().find(|&&x| x == sub_c) {
        Some(sub_c) => {
            let sub_m = sub_m.unwrap();

            let postman = Facteur::new(
                sub_m.value_of("DIRECTORY").unwrap(),
                matches.is_present("pretend")
            );

            match sub_c {
                &"init" => task::init(postman.git(sub_m.value_of("GIT_REPOSITORY").unwrap())),
                &"deploy" => task::deploy(postman.git(sub_m.value_of("GIT_REPOSITORY").unwrap())),
                &"rollback" => task::rollback(postman),
                _ => panic!("Impossible")
            }
        },
        None => println!("Please select a valid command. Use -h for help.")
    }
}
