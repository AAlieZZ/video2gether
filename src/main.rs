use std::io;
use std::cmp::Ordering;
use std::process::Command;

const PWD:&str="123";
const IP1:&str="192.168.1.1";
//const IP2=;

fn login(pwd:&str) -> bool {
    match pwd.cmp(PWD) {
        Ordering::Equal => true,
        Ordering::Less => false,
        Ordering::Greater => false,
    }
}

fn ssh() {
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("error");
    if choice.trim() == "1" {Command::new("ssh").arg("root@".to_string()+&IP1).output().expect("error");}
}

fn main() {
    let mut pwd = String::new();
    io::stdin().read_line(&mut pwd).expect("error");
    match login(&pwd.trim()) {
    true => ssh(),
    false => println!("error"),
    }
}
