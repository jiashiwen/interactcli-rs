use ssh2::Session;
use std::{io::Read, net::TcpStream};

pub fn main() {
    // Connect to the local SSH server
    let tcp = TcpStream::connect("114.67.127.196:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password("root", "Git785230").unwrap();

    let mut channel = sess.channel_session().unwrap();

    channel.exec("/tmp/interactcli-rs").unwrap();
    let mut s = String::new();

    channel.read_to_string(&mut s).unwrap();

    println!("s is: {}", s);
    channel.wait_close().ok();
    println!("{}", channel.exit_status().unwrap());
}
