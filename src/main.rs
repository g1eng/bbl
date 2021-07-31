use std::path::PathBuf;
use std::process::exit;
use std::fs;
use std::fs::Permissions;
use std::os::unix::fs::{PermissionsExt};
use clap::{App,Arg};
// use std::io::{BufReader, BufWriter};

struct Shell {
    path: PathBuf,
    // input: BufReader<String>,
    // output: BufWriter<String>,
    status: i32,
}

impl Shell {
    fn is_executable(&self) -> bool {
        if self.path.is_file() {
            println!("exist");
            let metadata = fs::metadata(&self.path);
            let permission = Permissions::from(metadata.unwrap().permissions());
            // println!("permission {:o}", permission.mode());
            let user_perm = (permission.mode() >> 6) % 8;
            // println!("user_perm {:o}", user_perm);
            user_perm == 7 || user_perm == 5
        } else {
            false
        }
    }

    fn parse(&mut self) -> Result<String, &'static str>{
        if self.is_executable() {
            Ok(String::from("executable"))
        } else {
            self.status = 1;
            Err("is not a executable")
        }
    }
}

fn main() {
    let app = App::new("riosh")
        .version("0.1.0")
        .author("Nomura Suzume")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .index(1)
                .help("executable to run")
                .required(true)
        )
        .get_matches();

    let mut sh = Shell{
        path: PathBuf::from(app.value_of("file").expect("invalid FILE specified")),
        status: 0
    };

    match sh.parse() {
        Ok(r) => println!("{:?}",r),
        Err(e) => {
            eprintln!("{:?}",e);
        }
    }

    exit(sh.status);
}


