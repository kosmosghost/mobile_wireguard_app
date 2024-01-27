use std::{fs, path::Path};
use rand::Rng;
use interfaces::Interface;

pub fn get_random_wg_config() -> String {
  let files = get_files();
  let num = generate_random_number(files.len());
  files[num].clone()
}

pub fn check_if_connection_active() -> Option<String>{
  let interfaces = get_interfaces();
  let files = get_files();
  let compared_interfaces = compare_interfaces(interfaces, files);
  compared_interfaces
}

fn generate_random_number(num: usize) -> usize {
  let secret_number: usize = rand::thread_rng().gen_range(1..num);
  secret_number
}


pub fn get_interface() -> String {
  let files = get_files();
  let interfaces = get_interfaces();
  /*
  See compare_interfaces()

  let interface = match compare_interfaces(interfaces, files) {
    Some(x) => x,
    None() => panic!("Can't find matching interfaces!"),
  };
  */
  let interface = compare_interfaces(interfaces, files);

  interface.unwrap()
}

fn get_files() -> Vec<String> {
  let mut files: Vec<String> = Vec::new();
  let path = fs::read_dir("/etc/wireguard").unwrap();
  for i in path {
    let file = i.unwrap().path().display().to_string();
    let filename = Path::new(&file).file_stem().unwrap().to_string_lossy().to_string();
    files.push(filename); 
  }

  files
}

fn get_interfaces() -> Vec<String> {
  let interface = Interface::get_all().unwrap();
  let mut interfaces: Vec<String> = Vec::new();
  for i in interface {
    interfaces.push(i.name.clone());
  }

  interfaces
}

/*
For some reason, which I'll never know the logic isn't working properly and for the sake
of my sanity I just gave up. I believe it is a bug in the rust compiler since it's a simple if /else
statement which is acting as both true and false at the same time.
This should return an Option<T> in the future.
*/
fn compare_interfaces(interfaces: Vec<String>, files: Vec<String>) -> Option<String> {
  let mut string_buffer: String = String::from("Null");
  for i in interfaces.clone() {
    for f in files.clone() {
      if i == f { 
        string_buffer = f.clone();
      }
    }
  }
  if string_buffer == "Null".to_string() {
    None
  }
  else {
    Some(string_buffer)
  }
}