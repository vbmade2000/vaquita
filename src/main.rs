mod scanner;

use scanner::scanner::scan;
use std::ops::Range;
use std::thread;

fn main() {
    let from = 1;
    let to = 65535;
    let host = String::from("127.0.0.1");
    let r = Range {
        start: from,
        end: to + 1,
    };
    println!("Open ports");
    println!("----------");
    for i in r {
        let _host = host.clone();
        thread::spawn(move || scan(_host, i as u32));
    }
}
