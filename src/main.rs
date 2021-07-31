use std::path::PathBuf;
use std::process::exit;
use std::fs;
use std::fs::{Permissions, File};
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use clap::{App,Arg};
use std::io::{BufReader, BufRead, self, Write};

struct Shell {
    path: PathBuf,
    command: Command,
    debug: bool,
    status: i32,
}

impl Shell {

    pub fn new() -> Shell {
        Shell{
            path: PathBuf::from(""),
            command: Command::new("ls"),
            debug: false,
            status: 0
        }
    }

    ///read_file reads file set in Shell.path and return BufReader<File> for it.
    fn read_file(&self) -> Result<BufReader<File>,()> {
        match File::open(&self.path) {
            Ok(f) => {
                Ok(BufReader::new(f))
            },
            Err(_) => Err(())
        }
    }

    ///is_executable detects Shell.path is a path of a executable or not
    fn is_executable(&self) -> bool {
        if self.path.is_file() {
            if self.debug {
                println!("[file exist]");
            }
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

    /// set_command sets command from a given String.
    /// This method is applied to the single lexical scope.
    /// ```rust
    /// use crates::Shell
    ///
    /// ```
    fn set_command(&mut self, line :String) -> Result<(),&str> {
        let mut arg_num = 0;
        for arg in line.split_ascii_whitespace() {
            if arg_num == 0 {
                self.command = Command::new(arg);
                arg_num += 1;
            } else {
                self.command.arg(arg);
                arg_num += 1;
            }
        }
        if arg_num == 0 {
            Err("no command")
        } else {
            Ok(())
        }
    }

    /// parse_command parses each lines for command execution
    fn parse_command(&mut self, line :String){
        self.set_command(line)
            .expect("failed to execute");
        let output = &self.command.output().expect("failed to execute");
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        println!("status: {}", &output.status);
    }

    /// read_lines reads lines from input file and make actions
    fn read_lines(&mut self) -> Result<String,()>{
        let reader = self.read_file()?;
        let mut lineno :i32 = 0;
        for line in reader.lines() {
            lineno += 1;
            let line = line.unwrap();
            if self.debug {
                println!("line[{}]: {:?}", lineno, line);
                self.parse_command(line)
            } else {
                self.parse_command(line)
                // println!("{:}", line.unwrap());
            }
        }
        if self.debug {
            println!("[total {} lines]", lineno);
        }
        Ok(String::from(""))
    }

    /// parse is the parser entry of the riosh virtual machine
    fn parse(&mut self) -> Result<String, &'static str>{
        if self.is_executable() {
            // Ok(String::from("executable"))
            match self.read_lines() {
                Ok(s) => Ok(s),
                Err(_) => Err("failed to parse lines")
            }
        } else {
            self.status = 1;
            Err("file is not an executable")
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
        .get_matches();


    let path;
    if app.is_present("file") {
        path = PathBuf::from(app.value_of("file").expect("invalid FILE specified"))
    } else {
        path = PathBuf::from("")
    }

    let mut sh = Shell{
        path,
        command: Command::new("ls"),
        debug: app.is_present("verbose"),
        status: 0
    };

    if app.is_present("file"){
        match sh.parse() {
            Ok(r) => print!("{:}",r),
            Err(e) => {
                eprintln!("{:?}",e);
            }
        }
    } else if app.is_present("command") {
        // println!("{:?}",app.value_of("command"));
        sh.parse_command(app.value_of("command").unwrap().to_string())
    } else {
        panic!("no such way of invocation")
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
            command: Command::new("ls"),
            debug: false,
            status: 0
        };
        shell
    }

    #[test]
    fn test_parse_no_error(){
        let mut shell = init("./fixtures/cmd");
        shell.parse().unwrap();
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

    #[test]
    fn test_new (){
        let sh = Shell::new();
    }
}