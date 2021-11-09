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

    // channel
    //     .exec("/root/interactcli-rs/target/release/interactcli-rs")
    //     .unwrap();
    let result = channel.exec("/root/interactcli-rs/target/release/interactcli-rs");
    println!("{:?}", result);
    let mut s = String::new();

    channel.read_to_string(&mut s).unwrap();

    println!("s is: {}", s);
    channel.wait_close();
    println!("{:?}", channel.exit_status());
}
