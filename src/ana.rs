use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::fmt::Debug;
use data::config::Config;
use data::Relation;
use rustc_serialize::Decodable;

pub fn exec() {
  ana::<Config>("src/config.json");
  ana::<Vec<Relation>>("src/rels.json");
  ana::<PathBuf>("src/path_buf.json");
  ana::<PathBuf>("src/not_founds.json");
}

fn ana<T: Debug + Decodable>(path_str: &str) -> Option<T> {
  let path = Path::new(path_str);
  let buf = read_file_contents(path)
    .or_else(|err| {
      println!("couldn't read file: {}", err);
      Err(err)
    })
    .ok()?;
  let s = ::rustc_serialize::json::decode::<T>(&buf)
    .or_else(|err| {
      println!("deserialisation error: {:?}", err);
      Err(err)
    })
    .ok()?;
  println!("{:?}", s);
  Some(s)
}

fn read_file_contents(path: &Path) -> io::Result<String> {
  let mut file = File::open(&path)?;
  let mut buf = String::new();
  file.read_to_string(&mut buf)?;
  println!("{}", buf);
  Ok(buf)
}
