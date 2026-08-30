use anyhow::Result;
use clap::Parser;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL to fetch
    url: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let url = args.url;
    let host = url.split("://").last().unwrap_or(&url).split('/').next().unwrap_or(&url);
    let path = if let Some(p) = url.find(host) { &url[p + host.len()..] } else { "/" };
    let port = if host.contains(":") { 80 } else { 80 };
    let host = host.split(':').next().unwrap_or(host);

    let addr = format!("{}:{}", host, port);
    let mut stream = TcpStream::connect_timeout(&addr.parse()?, Duration::from_secs(10))?;

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", path, host);
    stream.write_all(request.as_bytes())?;

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf)?;
    let response = String::from_utf8_lossy(&buf);

    for line in response.lines().take(20) {
        println!("{}", line);
    }
    Ok(())
}
