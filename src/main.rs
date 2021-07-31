use std::path::PathBuf;
use std::process::exit;
use std::fs;
use std::fs::Permissions;
use std::os::unix::fs::{PermissionsExt};

fn is_executable(desc :String) -> bool {
    let path = PathBuf::from(desc);
    let metadata = fs::metadata(path);
    let permission = Permissions::from(metadata.unwrap().permissions());
    // println!("permission {:o}", permission.mode());
    let user_perm = (permission.mode() >> 6) % 8;
    // println!("user_perm {:o}", user_perm);
    user_perm == 7 || user_perm == 5
}

fn is_file(desc :String) -> bool {
    let path = PathBuf::from(desc);
    path.is_file()
}

fn parse() -> Result<String, &'static str>{
    match is_file(String::from("ok")) {
        true => {
            println!("exist");
            if is_executable(String::from("ok")) {
                Ok(String::from("executable"))
            } else {
                Err("is not a file")
            }
        },
        false => {
            Err("no there")
        }
    }
}

fn main() {
    match parse() {
        Ok(r) => println!("{:?}",r),
        Err(e) => {
            eprintln!("{:?}",e);
            exit(1)
        }
    }
}
