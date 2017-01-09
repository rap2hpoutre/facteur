# Facteur
Facteur helps you to install, deploy,  and rollbak Laravel applications.

## Usage

```
USAGE:
    facteur [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -p, --pretend    Run a simulation (don't do anything)
    -v               Sets the level of verbosity
    -V, --version    Prints version information

SUBCOMMANDS:

    help        Prints this message or the help of the given subcommand(s)
    init        Initialization of a Laravel application
    deploy      Deploy a Laravel application (previously initialized)
    rollback    Rollback to previous release
 ```
 
### Init
Initialization of a Laravel application
 ```
USAGE:
    facteur init <DIRECTORY> <GIT_REPOSITORY>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <DIRECTORY>         The directory of your web app
    <GIT_REPOSITORY>    Sets the input file to use
```
    
### Deploy
Deploy a Laravel application (previously initialized)
    
```
USAGE:
    facteur deploy <DIRECTORY> <GIT_REPOSITORY>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <DIRECTORY>         The directory of your web app
    <GIT_REPOSITORY>    Sets the input file to use
```

### Rollback

Rollback to previous release
    
 ```
USAGE:
    facteur rollback <DIRECTORY>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <DIRECTORY>    The directory of your web app
```
