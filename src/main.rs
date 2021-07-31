use std::path::PathBuf;
use std::process::exit;
use std::fs;
use std::fs::{Permissions, File};
use std::os::unix::fs::PermissionsExt;
use clap::{App,Arg};
use std::io::{BufReader, BufWriter, BufRead};

struct Shell {
    path: PathBuf,
    debug: bool,
    status: i32,
}

impl Shell {

    fn read_file(&self) -> Result<BufReader<File>,()> {
        match File::open(&self.path) {
            Ok(f) => {
                Ok(BufReader::new(f))
            },
            Err(_) => Err(())
        }
    }

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
    fn parse_lines(&self) -> Result<String,()>{
        let reader = self.read_file()?;
        let mut lineno :i32 = 0;
        for line in reader.lines() {
            lineno += 1;
            println!("line[{}]: {:?}", lineno, line.unwrap());
        }
        println!("total {} lines", lineno);
        Ok(String::from(""))
    }

    fn parse(&mut self) -> Result<String, &'static str>{
        if self.is_executable() {
            // Ok(String::from("executable"))
            match self.parse_lines() {
                Ok(s) => Ok(s),
                Err(_) => Err("kore ")
            }
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
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .help("show debug messages")
                .required(false)
        )
        .get_matches();

    let mut sh = Shell{
        path: PathBuf::from(app.value_of("file").expect("invalid FILE specified")),
        debug: app.value_of("verbose") == "true",
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Shell;

    fn init(s :&str) -> Shell {
        let shell = Shell{
            path: PathBuf::from(s),
            debug: bool,
            status: 0
        };
        shell
    }

    #[test]
    fn test_parse_executable(){
        let mut shell = init("./fixtures/ok");
        assert_eq!(shell.parse().unwrap(), String::from("executable"))
    }

    #[test]
    #[should_panic]
    fn test_parse_error(){
        let mut shell = init("./fixtures/utopia/no/there");
        shell.parse().unwrap();
    }

    #[test]
    fn test_is_executable_true(){
        let mut shell_ok = init("./fixtures/ok");
        assert_eq!(shell_ok.is_executable(), true)
    }

    #[test]
    fn test_is_executable_false(){
        let mut shell_ok = init("./fixtures/doco/nimo/9");
        assert_eq!(shell_ok.is_executable(), false)
    }
}