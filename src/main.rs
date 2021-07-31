use std::path::PathBuf;
use std::process::exit;
use std::fs;
use std::fs::Permissions;
use std::os::unix::fs::{PermissionsExt};
use clap::{App,Arg};

fn is_executable(desc :String) -> bool {
    let path = PathBuf::from(desc);
    if path.is_file() {
        println!("exist");
        let metadata = fs::metadata(path);
        let permission = Permissions::from(metadata.unwrap().permissions());
        // println!("permission {:o}", permission.mode());
        let user_perm = (permission.mode() >> 6) % 8;
        // println!("user_perm {:o}", user_perm);
        user_perm == 7 || user_perm == 5
    } else {
        false
    }
}

fn is_file(desc :String) -> bool {
    let path = PathBuf::from(desc);
    path.is_file()
}

fn parse(file :String) -> Result<String, &'static str>{
    if is_executable(file) {
        Ok(String::from("executable"))
    } else {
        Err("is not a executable")
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


    match parse(String::from(app.value_of("file").unwrap())) {
        Ok(r) => println!("{:?}",r),
        Err(e) => {
            eprintln!("{:?}",e);
            exit(1)
        }
    }
}
