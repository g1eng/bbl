mod shell;

use std::path::PathBuf;
use std::process::exit;
use clap::{App, Arg, ArgMatches};
use std::io::stdin;
use shell::Shell;


/// get_app_matcher returns default bbl clap matcher
fn get_app_matcher() -> ArgMatches<'static> {
    App::new("bbl")
        .version("0.1.0")
        .author("Nomura Suzume")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .index(1)
                .help("executable to run")
                .required(false)
        )
        .arg(
            Arg::with_name("command")
                .short("c")
                .help("execute command line with argument string")
                .value_name("COMMAND_LINE")
                .takes_value(true)
                .required(false)
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .help("show debug messages")
                .required(false)
        )
        .get_matches()
}

fn main() {
    let app = get_app_matcher();
    let path;

    if app.is_present("file") {
        path = PathBuf::from(app.value_of("file").expect("invalid FILE specified"))
    } else {
        path = PathBuf::from("")
    }

    let mut sh = Shell::new();
    sh.debug = app.is_present("verbose");
    sh.path = path;

    if app.is_present("file"){
        match sh.parse() {
            Ok(r) => print!("{:}",r),
            Err(e) => {
                eprintln!("{:?}",e);
            }
        }
    } else if app.is_present("command") {
        // println!("{:?}",app.value_of("command"));
        sh.parse_command(
            app.value_of("command")
                .unwrap()
                .to_string()
        )
    } else {
        sh.read_lines(stdin().lock());
    }

    exit(sh.status);
}
