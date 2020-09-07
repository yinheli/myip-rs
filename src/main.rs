use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::io::Read;
use std::process;

fn main() {
    let addr = "ns1.dnspod.net:6666".to_socket_addrs().unwrap().next().unwrap();
    let tcp = TcpStream::connect_timeout(&addr, Duration::new(5, 0));
    let mut tcp = match tcp {
        Ok(t) => t,
        Err(e) => {
            println!("error: {}", e);
            process::exit(1);
        }
    };
    let mut buf = String::new();
    tcp.set_read_timeout(Option::Some(Duration::new(5, 0))).unwrap();
    let size = tcp.read_to_string(&mut buf).unwrap();
    if size > 0 {
        println!("{}", buf);
    } else {
        println!("read error");
        process::exit(1);
    }
}
