use std::process::Command;

pub fn run_wq_quick_up(interface: String) {
    let mut buffer = String::from("wg-quick up ");
    buffer += &interface;
    Command::new("sh").arg("-c").arg(buffer).output().unwrap();
}

pub fn run_wq_quick_down(interface: String) {
    let mut buffer = String::from("wg-quick down ");
    buffer += &interface;
    Command::new("sh").arg("-c").arg(buffer).output().unwrap();
}