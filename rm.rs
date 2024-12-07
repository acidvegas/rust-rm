use std::process::Command;
use std::env;

fn main() {
   let args: Vec<String> = env::args().skip(1).collect();
   Command::new("rm").args(args).spawn().unwrap();
}