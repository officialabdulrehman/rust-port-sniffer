use std::{
    env,
    io::{self, Write},
    net::{IpAddr, TcpStream},
    process,
    str::FromStr,
    sync::mpsc::{channel, Sender},
    thread,
};

const END_PORT: u16 = 65535;

#[derive(Debug)]
struct Input {
    flag: String,
    ip: IpAddr,
    threads: u16,
}

impl Input {
    fn new(args: &[String]) -> Result<Input, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        } else {
            let first_arg = args[1].clone();
            if let Ok(ip) = IpAddr::from_str(&first_arg) {
                return Ok(Input {
                    flag: String::from(""),
                    ip,
                    threads: 8,
                });
            } else {
                let flag = args[1].clone();
                if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                    println!(
                        "Usage: -j to select how many threads you want
              \n\r       -h or -help to show this help message"
                    );
                    return Err("help");
                } else if flag.contains("-h") || flag.contains("-help") {
                    return Err("too many arguments");
                } else if flag.contains("-j") {
                    let ip = match IpAddr::from_str(&args[3]) {
                        Ok(s) => s,
                        Err(_) => return Err("not a valid IP address; must be IPv4 or IPv6"),
                    };
                    let threads = match args[2].parse::<u16>() {
                        Ok(s) => s,
                        Err(_) => return Err("failed to parse thread number"),
                    };
                    return Ok(Input { threads, flag, ip });
                } else {
                    return Err("invalid syntax");
                }
            }
        }
    }
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
    let raw_args: Vec<String> = env::args().collect();
    // let args = Input {
    //     flag: String::from("-i"),
    //     ip: IpAddr::from_str("192.168.10.1").unwrap(),
    //     threads: 4,
    // };
    let args = Input::new(&raw_args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("failed to parse the arguments: {}", err);
            process::exit(0);
        }
    });
    println!("executing => {} {} {}", args.flag, args.ip, args.threads);
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
