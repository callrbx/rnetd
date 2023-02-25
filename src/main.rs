use std::net::{IpAddr, SocketAddr, TcpListener};
use std::os::fd::{AsRawFd, FromRawFd};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::FromStr;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(
        short = "p",
        long = "port",
        help = "port to listen on",
        default_value = "1337"
    )]
    port: u16,
    #[structopt(
        short = "i",
        long = "ip",
        help = "ip to listen on",
        default_value = "0.0.0.0"
    )]
    ip: String,
    #[structopt(help = "binary to run")]
    binary: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::from_args();

    let ip = match IpAddr::from_str(&args.ip) {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("Invalid IP");
            std::process::exit(1);
        }
    };

    let addr = SocketAddr::from((ip, args.port));
    let listener = TcpListener::bind(addr)?;

    println!("Running {:?} on {}", args.binary, args.port);

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Got Connection - Execing");
                let fname = args.binary.clone();
                let fd_in = stream.as_raw_fd();
                let fd_out = stream.as_raw_fd();
                let mut proc = Command::new(fname)
                    .stdin(unsafe { Stdio::from_raw_fd(fd_in) })
                    .stdout(unsafe { Stdio::from_raw_fd(fd_out) })
                    .spawn()
                    .expect("Failed to start process");
                proc.wait()?;
            }
            Err(_) => todo!(),
        }
    }

    return Ok(());
}
