use std::{
    env,
    io::{self, Write},
    net::{IpAddr, TcpStream},
    str::FromStr,
    sync::mpsc::{channel, Sender},
    thread,
};

const END_PORT: u16 = 65535;

struct Input {
    flag: String,
    ip: IpAddr,
    threads: u16,
}

fn scan(tx: Sender<u16>, start_port: u16, ip: IpAddr, threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((ip, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }
        if (END_PORT - port) <= threads {
            break;
        }
        port += threads
    }
}

fn main() {
    // let rawArgs: Vec<String> = env::args().collect();
    let args = Input {
        flag: String::from("-i"),
        ip: IpAddr::from_str("192.168.10.1").unwrap(),
        threads: 4,
    };
    let (tx, rx) = channel();

    for i in 0..args.threads {
        let tx = tx.clone();
        thread::spawn(move || scan(tx, i, args.ip, args.threads));
    }

    let mut result = vec![];

    drop(tx);

    for p in rx {
        result.push(p);
    }

    println!("");

    result.sort();

    for port in result {
        println!("{} is open", port);
    }
}
