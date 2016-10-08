#[macro_use]
extern crate clap;

use clap::App;
use clap::Arg;

use std::io::copy;
use std::io::stdin;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;
use std::fs::OpenOptions;
use std::thread;
use std::process;
use std::sync::Arc;
use std::sync::Mutex;


fn main() {

  let matches = App::new("tfw")
    .version(crate_version!())
    .author(crate_authors!())
    .about("Shell utility for conditional redirection")
    .arg(Arg::with_name("switch")
       .value_name("SWITCH")
       .help("File whose last line indicates where to direct the input.")
       .takes_value(true)
       .required(true)
       .multiple(false))
    .get_matches();

  let switch = unwrap(
    matches.value_of("switch")
    .map(|path| OpenOptions::new().read(true).open(path))
    .unwrap());

  let target = Arc::new(Mutex::new(None));

  {
    let target = target.clone();
    thread::spawn(move || {
      let files =
        BufReader::new(switch)
          .lines().map(unwrap)
          .map(|path|
            OpenOptions::new()
              .create(true)
              .append(true)
              .open(path)).map(unwrap);
      for file in files {
        *(target.lock().unwrap()) = Some(file);
      }
    });
  }

  let stdin = stdin();
  let lines = stdin.lock().lines().map(unwrap);

  while target.lock().unwrap().is_none() {};

  for mut line in lines {
    line.push('\n');
    unwrap(copy(
      &mut line.as_bytes(),
      (*target.lock().unwrap()).as_mut().unwrap()));
  }
}


fn unwrap<T>(v : Result<T>) -> T {
  v.unwrap_or_else(|e| {
    println!("{}", e);
    process::exit(1)
  })
}
