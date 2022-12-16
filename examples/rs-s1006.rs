// Bind to all interfaces RS-S1006
// Security
// Binding to all network interfaces can potentially open up a service to traffic on unintended interfaces.

// Acccording to their specifications, the following addresses are "unspecified" addresses:

// 0.0.0.0 in IPv4
// 0:0:0:0:0:0 (or just ::) in IPv6
// When you bind a socket to all interfaces using an "unspecified" address as the IP address, you essentially allow it to accept connections from any IP address provided, that can get to the socket via routing. Binding to all interfaces is therefore associated with security risks and is not recommended.

// BAD PRACTICE
// use std::net::TcpListener;

// let listener = TcpListener::bind("0.0.0.0:80")?;
// RECOMMENDED
// use std::net::TcpListener;

// let listener = TcpListener::bind("127.0.0.1:80")?;
// REFERENCES
// The Unspecified Address
// CVE-2018-1281
// CWE-200: Exposure of Sensitive Information to an Unauthorized Actor
fn main() {
    use std::net::TcpListener;
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
}