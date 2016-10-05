use std::io;
use std::env;
use std::thread;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::OpenOptions;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicUsize;

fn main() {

  let files = env::args().
    skip(2).
    map(|path|
      OpenOptions::new().
        read(false).
        write(true).
        create(true).
        open(path)).
    map(Result::unwrap).
    collect::<Vec<File>>();

  let target = Arc::new(AtomicUsize::new(0));
  let target_writer = target.clone();

  thread::spawn(move || {
    for line in BufReader::new(
        env::args().nth(1).
        map(|path|
          OpenOptions::new().
            read(true).
            write(false).
            create(false).
            open(path)).
        unwrap().unwrap()).
        lines() {
      target_writer.store(
        line.unwrap().parse::<usize>().unwrap_or(
          target_writer.load(Ordering::Relaxed)),
        Ordering::Relaxed);
    }
  });

  let stdin = io::stdin();
  let mut buffer = stdin.lock();

  loop {
    let consumed = buffer.fill_buf().
      map(|bytes| 
        files.get(target.load(Ordering::Relaxed)).
      map(|mut file| file.write(bytes))).
      unwrap().unwrap().unwrap();
    buffer.consume(consumed);
  }
}
