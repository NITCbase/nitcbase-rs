#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

use std::env;
mod Algebra;
mod BPlusTree;
mod BlockAccess;
mod Buffer;
mod Cache;
mod Disk;
mod Frontend;
mod FrontendInterface;
mod RegexHandler;
mod Schema;
mod define;

fn main() -> () {
    Disk::new_disk().expect("Disk Error. Please  check ../Disk/disk file");
    let args: Vec<String> = env::args().collect();
    let argc = args.len();
    let argv = args.iter().map(|arg| arg.as_str()).collect::<Vec<&str>>();

    let res = FrontendInterface::handleFrontend(argc, &argv);
    match res {
        Ok(_) => println!("Closing."),
        Err(_) => println!("Readline threw error"),
    };
}
