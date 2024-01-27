use std::{fs, path::Path};
use rand::Rng;
use interfaces::Interface;

pub fn get_random_wg_config() -> String {
  let files = get_files();
  let num = generate_random_number(files.len());
  files[num].clone()
}

fn generate_random_number(num: usize) -> usize {
  let secret_number: usize = rand::thread_rng().gen_range(1..num);
  secret_number
}


pub fn get_interface() -> String {
  let files = get_files();
  let interfaces = get_interfaces();
  compare_interfaces(interfaces, files)
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

fn compare_interfaces(interfaces: Vec<String>, files: Vec<String>) -> String {
  let mut string_buffer: String = String::new();
  for i in interfaces.clone() {
    for f in files.clone() {
      if i == f { 
        string_buffer = f.clone();
      }
    }
  }

  string_buffer
}