use std::path::Path;
use std::fs;
use std::io;
use std::env;
use filetime::{ set_file_mtime, FileTime };

pub fn is_exists(fname: &str) -> bool {
    Path::new(fname).exists()
}

pub fn is_file(fname: &str) -> bool {
    Path::new(fname).is_file()
}

pub fn is_dir(fname: &str) -> bool {
    Path::new(fname).is_dir()
}

pub fn listdir(fname: &str) -> Result<Vec<String>, io::Error> {
    let mut v = vec![];
    let files = fs::read_dir(fname)?;

    for file in files {
        v.push(file.unwrap().path().file_name().unwrap()
            .to_os_string().into_string().unwrap());
    }

    Ok(v)
}

pub fn touch(fname: &str) -> Result<bool, io::Error> {
    if !is_exists(fname) {
        fs::File::create(fname)?;
        return Ok(false);
    }

    set_file_mtime(fname, FileTime::now())?;

    Ok(true)
}

pub fn rm(fname: &str) -> io::Result<()> {
    fs::remove_file(fname)?;
    Ok(())
}

pub fn rmdir(fname: &str) -> io::Result<()> {
    fs::remove_dir(fname)?;
    Ok(())
}

pub fn rmdirq(fname: &str) -> io::Result<()> {
    if !is_exists(fname) {
        return Ok(());
    }

    rmdir(fname)?;
    Ok(())
}

pub fn rmdirp(fname: &str) -> io::Result<()> {
    fs::remove_dir_all(fname)?;
    Ok(())
}

pub fn rmdirpq(fname: &str) -> io::Result<()> {
    if !is_exists(fname) {
        return Ok(());
    }
    
    rmdirp(fname)?;
    Ok(())
}

pub fn mkdir(fname: &str) -> io::Result<()> {
    fs::create_dir(fname)?;
    Ok(())
}

pub fn mkdirq(fname: &str) -> io::Result<()> {
    if is_exists(fname) {
        return Ok(());
    }

    mkdir(fname)?;
    Ok(())
}

pub fn mkdirp(fname: &str) -> io::Result<()> {
    fs::create_dir_all(fname)?;
    Ok(())
}

pub fn mkdirpq(fname: &str) -> io::Result<()> {
    if is_exists(fname) {
        return Ok(());
    }

    mkdirp(fname)?;
    Ok(())
}

#[derive(Clone)]
pub struct Argument {
    short_opt: String,
    long_opt: String,
    action: String,
    bool_val: bool,
    i32_val: i32,
    i64_val: i64,
    f32_val: f32,
    f64_val: f64,
    usize_val: usize,
    string_val: String,
}

#[derive(Clone)]
pub struct ProgramArguments {
    add_args: Vec<Argument>,
    parsed_args: Vec<Argument>,
    args: Vec<String>
}

impl ProgramArguments {
    pub fn new() -> Self {
        ProgramArguments {
            add_args: vec![],
            parsed_args: vec![],
            args: vec![],
        }
    }

    pub fn add_arg(
        &mut self,
        short_opt: &str,
        long_opt: &str,
        action: &str,
    ) {
        let add_arg = Argument {
            short_opt: short_opt.to_string(),
            long_opt: long_opt.to_string(),
            action: action.to_string(),
            bool_val: false,
            i32_val: 0,
            i64_val: 0,
            f32_val: 0.0,
            f64_val: 0.0,
            usize_val: 0,
            string_val: String::from(""),
        };
        self.add_args.push(add_arg)
    }

    pub fn parse_args(&mut self) -> &Self {
        let args = env::args().collect();

        self.parse(&args);

        self
    }

    pub fn parse(&mut self, args: &Vec<String>) {
        let mut i = 0;

        while i < args.len() {
            let mut arg = &args[i];
            i += 1;
            let result: Option<Argument> = self.match_arg(&arg);
            let mut add_arg = match result {
                Some(add_arg) => add_arg,
                None => { 
                    continue;
                }
            };

            match add_arg.action.as_str() {
                "store_true" => add_arg.bool_val = true,
                "store_false" => add_arg.bool_val = false,
                "store_i32" => {
                    if i < args.len() {
                        arg = &args[i];
                        i += 1;
                        add_arg.i32_val = arg.parse().unwrap();
                    }
                },
                "store_i64" => {
                    if i < args.len() {
                        arg = &args[i];
                        i += 1;
                        add_arg.i64_val = arg.parse().unwrap();
                    }
                },
                "store_f32" => {
                    if i < args.len() {
                        arg = &args[i];
                        i += 1;
                        add_arg.f32_val = arg.parse().unwrap();
                    }
                },
                "store_f64" => {
                    if i < args.len() {
                        arg = &args[i];
                        i += 1;
                        add_arg.f64_val = arg.parse().unwrap();
                    }
                },
                "store_usize" => {
                    if i < args.len() {
                        arg = &args[i];
                        i += 1;
                        add_arg.usize_val = arg.parse().unwrap();
                    }
                },
                "store_string" => {
                    if i < args.len() {
                        arg = &args[i];
                        i += 1;
                        add_arg.string_val = arg.clone();
                    }
                },
                &_ => {
                    self.args.push(arg.clone());
                    continue;
                }
            }

            self.parsed_args.push(add_arg);
        }
    }

    fn match_arg(&mut self, arg: &String) -> Option<Argument> {
        for i in 0..self.add_args.len() {
            let add_arg = &self.add_args[i];
            if add_arg.short_opt == *arg || 
               add_arg.long_opt == *arg {
                return Some(add_arg.clone());
            }
        }

        None
    }

    pub fn get_arg(&self, index: usize) -> Option<&String> {
        if index >= self.args.len() {
            return None;
        }

        Some(&self.args[index])
    }

    pub fn get_opt(&self, opt: &str) -> Option<&Argument> {
        for parsed_arg in self.parsed_args.iter() {
            if parsed_arg.short_opt.as_str() == opt ||
               parsed_arg.long_opt.as_str() == opt  {
                return Some(&parsed_arg);
            }
        }

        None
    }
}

pub fn parse_args() -> ProgramArguments {
    let mut parg = ProgramArguments::new();
    parg.parse_args();
    parg
}
