//! TCP and UDP Echo Servers.
//!
//! Implementation of [RFC 862](https://tools.ietf.org/html/rfc862)
extern crate clap;

use std::process;

mod tcp;


/// An EchoServer instance must be able to handle clients.
pub trait EchoServer {
    fn start(&self, host: &str, port: u16) -> Result<(), String>;
}


fn main() {

    let args = clap::App::new("echo-server")
                         .version("0.1.0")
                         .author("Danilo Bargen <mail@dbrgn.ch>")
                         .about("Implementation of RFC 862")
                         .arg(clap::Arg::with_name("port")
                                        .short("p")
                                        .long("port")
                                        .value_name("PORT")
                                        .default_value("7")
                                        .help("Override the port to listen on"))
                         .get_matches();
    let port = args.value_of("port").expect("Port not set")
                                    .parse::<u16>().unwrap_or_else(|e| {
                                        println!("Invalid port: {}. Must be an integer.", e);
                                        process::exit(1);
                                    });

    let server = tcp::TcpEchoServer;
    server.start("127.0.0.1", port).unwrap_or_else(|e| {
        println!("Error: {}", e);
        process::exit(1);
    });

}
