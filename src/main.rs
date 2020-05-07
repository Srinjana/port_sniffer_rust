use std::env;
use std::io::{Write,self};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::thread;
use std::sync::mpsc::{Sender, channel};

const MAX: u16 = 65535;  //max port that we can sniff

struct Arguments {
    flag: String, 
    ipaddr: IpAddr, 
    threads: u16,
}

impl Arguments 
{
    fn new(args: &[String]) -> Result<Arguments, &'static str> //send back errors to main fn and have them checked

    {
        if args.len() < 2 {
            return Err("not enough arguments found");
        }
        else if args.len() > 4 {
            return Err("too many arguments found");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f)  //destruct ip from string
        {
            return Ok(Arguments {flag: String::from(""), ipaddr, threads: 4});
        }
        else {
            let flag = args[1].clone();    //either no ip (send help) or some error
            //if any other agruments with help
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 
                {
                    println!("Usage: -j to select the number of threads you want
                    \n\r       -h or -help to display the help message");
                    return Err("help");
                }
            else if flag.contains("-h") || flag.contains("-help") 
                {
                    return Err("Too many arguments found.");
                }
            else if flag.contains("-j") 
                {
                    let ipaddr = match IpAddr::from_str(&args[3])
                    {
                        Ok(s) => s,
                        Err(_) => return Err("Not a valid Ip Adress; must be IPv4 or IPv6")
                    };
                    let threads = match args[2].parse::<u16>()
                    {
                        Ok(s) => s,
                        Err(_) => return Err("Failed to parse the thread number")
                    };
                    return Ok(Arguments{threads, flag, ipaddr});
                }
            else 
            {
                return Err("invalid syntax.");
            }
            }
    }
}


fn scan (tx: Sender<u16>, start_port: u16, addr: IpAddr, num_thread: u16)
{

    let mut port: u16 = start_port + 1;
    loop 
    {
        match TcpStream::connect((addr, port))
        {
            Ok(_) => 
            {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}    
            
        }

        if (MAX - port) <= num_thread
        {
            break;
        }

        port += num_thread;
        
    }
}


fn main() {
   
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else
    (
         |err| {
            if err.contains("help") {
                process::exit(0);  //to avoid panic
            } 
            else {
                //error printnew line will display all errors written above and exit the  process
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(0);
            }
        }
    );

    let num_thread = arguments.threads;
    let addr = arguments.ipaddr;
    //transmittter and receiver
    let (tx, rx) = channel();
    for i in 0..num_thread {
        let tx = tx.clone();

         //i is no. of thread
         thread::spawn(move || {
            scan(tx, i, addr, num_thread);
        });

    }

    let mut out = vec![];
    drop(tx);
    for p in rx 
    {
        out.push(p);
    }    
    
    println!("");
    out.sort();
    for v in out 
    {
        println!("{} is open. ", v);
    }
}

/*port_sniffer.exe -h                    //provides a help screen
port_sniffer.exe -j 100 192.168.1.1    // allows user to set the number of threads they want to use
port_sniffer.exe 192.168.1.1           // calling a tool to a default ip with set number of threads.*/